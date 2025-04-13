// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod aes_cipher;
mod config;
mod convert_preference;
mod ical_config;
mod ical_downloader;
mod ical_reader;
mod ical_writer;
mod logger;
mod paths;
mod primitive;

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::{
    command,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};
use tauri_plugin_notification::NotificationExt;
use tokio::time;

#[derive(Deserialize, Serialize, Clone)]
struct ToastNotice {
    title: String,
    body: String,
}

#[command]
async fn send_ical_url(
    url: String,
    notice_timing: String,
    save_config: bool,
) -> Result<String, String> {
    let new_config = ical_config::Config {
        ical: ical_config::Cal {
            ical_url: url,
            notice_timing: notice_timing,
        },
    };

    let client = Client::new();

    match ical_writer::download_ical(&new_config.ical.ical_url, save_config, &client).await {
        Ok(ical_temp_path) => {
            if let Some(path_str) = ical_temp_path.to_str() {
                match ical_reader::read_ical(path_str) {
                    Ok(_) => {
                        if save_config {
                            if let Err(e) = config::update_config(&new_config) {
                                return Err(format!("設定の保存に失敗しました。: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        return Err(format!("{}", e));
                    }
                }

                if !save_config {
                    let _ = fs::remove_file(&ical_temp_path);
                }
            } else {
                return Err("iCalendarの読込に失敗しました。".to_string());
            }
        }
        Err(e) => {
            return Err(format!("{}", e));
        }
    }

    Ok("iCal URLのテストに成功しました。".to_string())
}

#[command]
fn get_initial_params() -> ical_config::Cal {
    let config_guard = ical_config::get_config();

    config_guard.ical.clone()
}

#[command]
fn notice_toast(app: tauri::AppHandle, title: &str, body: &str) {
    match app
        .notification()
        .builder()
        .title(title)
        .body(body.replace("\\r", " ").replace("\\n", " "))
        .show()
    {
        Ok(_) => {}
        Err(e) => logger::logging_error(&format!("トースト通知に失敗しました。: {}", e)),
    }

    // フロント側にイベント発行する
    // let payload = ToastNotice {
    //     title: title.to_string(),
    //     body: body.replace("\\r", " ").replace("\\n", " ").to_string(),
    // };

    // app.emit("event-listen", payload).unwrap_or_else(|e| {
    //     logger::logging_error(&format!("トースト通知に失敗しました。: {}", e));
    // });
}

async fn notify_upcoming_events(
    app: tauri::AppHandle,
    cal_infos: Vec<ical_reader::CalInfo>,
    now: DateTime<FixedOffset>,
    second: i64,
) {
    for info in cal_infos {
        match primitive::parse_datetime(&info.date_start) {
            Ok(date) => {
                let duration = primitive::calculate_duration(now, date);

                let s = duration.num_seconds();

                if s > second - 60 && s <= second {
                    let start = primitive::convert_datetime_2_timeonly(&info.date_start)
                        .unwrap_or_default();
                    let end =
                        primitive::convert_datetime_2_timeonly(&info.date_end).unwrap_or_default();

                    let title = format!("[{}-{}] {}", &start, &end, &info.summary);

                    notice_toast(app.clone(), &title, &info.description);
                }
            }
            Err(e) => {
                logger::logging_error(&e);
            }
        }
    }
}

async fn start_ical_timer(app: tauri::AppHandle) -> Result<(), String> {
    let total_millisecond = primitive::get_total_millisecond_2_next_minute();
    //debug
    //println!("次の分までのミリ秒数: {}", total_millisecond);

    tokio::spawn(async move {
        time::sleep(time::Duration::from_millis(total_millisecond as u64)).await;

        let client = Client::new();

        let mut interval = time::interval(time::Duration::from_secs(60));
        loop {
            interval.tick().await;

            //debug
            //println!("1分経過しました。");
            let now_timer = primitive::get_current_datetime_jst();
            //debug
            //println!("日時: {}", now_timer);

            let (ical_url, notice_timing) = {
                let config = Arc::new(Mutex::new(ical_config::get_config().clone()));

                let config_guard = config.lock().expect("CONFIG のロックに失敗しました。");
                (
                    config_guard.ical.ical_url.clone(),
                    config_guard.ical.notice_timing.clone(),
                )
            };

            let second = convert_preference::convert_timing_2_minutes(&notice_timing);

            match ical_writer::download_ical(&ical_url, true, &client).await {
                Ok(ical_temp_path) => {
                    if let Some(path_str) = ical_temp_path.to_str() {
                        match ical_reader::read_ical(path_str) {
                            Ok(cal_infos) => {
                                notify_upcoming_events(app.clone(), cal_infos, now_timer, second)
                                    .await;
                            }
                            Err(e) => {
                                logger::logging_error(&e);
                            }
                        }
                    } else {
                        logger::logging_error("iCalendar の読込に失敗しました。");
                    }
                }
                Err(e) => {
                    logger::logging_error(&e);
                }
            }
        }
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("'main' ウィンドウが存在しません。")
                .set_focus();
        }))
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let separator = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "設定", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &separator, &quit])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            // "main" ウィンドウの取得
            let main_window = app.get_webview_window("main").unwrap();

            let _ = logger::initialize_logger();

            if config::ensure_config_file()? {
                main_window.hide().unwrap();
            }

            tauri::async_runtime::spawn({
                let app_handle = app.handle().clone(); // ここでクローンを作成

                async move {
                    if let Err(e) = start_ical_timer(app_handle).await {
                        logger::logging_error(&format!(
                            "start_ical_timer 関数の実行中にエラーが発生しました。: {}",
                            e
                        ));
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_ical_url,
            get_initial_params,
            notice_toast
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
