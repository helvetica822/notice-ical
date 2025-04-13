use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub ical: Cal,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cal {
    pub ical_url: String,
    pub notice_timing: String,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config {
        ical: Cal {
            ical_url: String::new(),
            notice_timing: "20".to_string(),
        },
    });
}

pub fn get_config() -> std::sync::MutexGuard<'static, Config> {
    CONFIG.lock().expect("CONFIG のロックに失敗しました。")
}

pub fn set_config(new_config: &Config) {
    let mut config = CONFIG.lock().expect("CONFIG のロックに失敗しました。");
    *config = new_config.clone();
}
