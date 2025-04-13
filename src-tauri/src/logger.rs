use crate::paths;
use crate::primitive;

use log::{error, info};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub fn initialize_logger() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir_path = paths::get_log_path();

    fs::create_dir_all(&log_dir_path)?;
    cleanup_logs_older_than_one_month(&log_dir_path)?;

    let now = primitive::get_current_datetime_jst();

    let log_file_name = format!("{}.log", now.format("%Y-%m-%d").to_string());
    let log_file_path = log_dir_path.join(log_file_name);

    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&log_file_path)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}]: {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(log_file)
        .apply()?;

    Ok(())
}

pub fn logging_info(log: &str) {
    let log_with_time = get_log_with_time(&log);
    info!("{}", &log_with_time);
}

pub fn logging_error(log: &str) {
    let log_with_time = get_log_with_time(&log);
    error!("{}", &log_with_time);
}

fn get_log_with_time(log: &str) -> String {
    let now = primitive::get_current_datetime_jst();
    let t = now.format("%H:%M:%S").to_string();

    format!("{}: {}", t, log)
}

fn cleanup_logs_older_than_one_month(dir_path: &Path) -> std::io::Result<()> {
    let one_month_ago = SystemTime::now() - Duration::new(30 * 24 * 60 * 60, 0);

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let modified_time = metadata.modified()?;

            if modified_time < one_month_ago {
                fs::remove_file(entry.path())?;
            }
        }
    }

    Ok(())
}
