use crate::ical_downloader;
use crate::paths;

use reqwest::Client;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

pub async fn download_ical(ical_url: &str, save: bool, client: &Client) -> Result<PathBuf, String> {
    let body = ical_downloader::fetch_ical(ical_url, &client)
        .await
        .map_err(|e| format!("{}", e))?;

    let file_name = get_file_name(save);
    let ical_temp_path = paths::get_appdata_path().join(file_name);

    save_ical_to_file(ical_temp_path.clone(), &body)
        .map_err(|e| format!("iCalendar の保存に失敗しました。: {}", e))?;

    Ok(ical_temp_path)
}

fn save_ical_to_file(file_path: PathBuf, content: &str) -> io::Result<()> {
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn get_file_name(save: bool) -> String {
    if save {
        String::from("tmp.ical")
    } else {
        String::from("test.ical")
    }
}
