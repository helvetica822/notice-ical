use crate::aes_cipher;
use crate::convert_preference;
use crate::ical_config;
use crate::logger;
use crate::paths;

use std::fs;
use std::fs::File;
use std::io::Write;

fn read_config(file_path: &str) -> Result<ical_config::Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: ical_config::Config = toml::from_str(&contents)?;

    let dec_config = ical_config::Config {
        ical: ical_config::Cal {
            ical_url: aes_cipher::aes_decrypt(&config.ical.ical_url)?,
            notice_timing: convert_preference::check_timing_value(&config.ical.notice_timing),
        },
    };

    ical_config::set_config(&dec_config);

    Ok(dec_config)
}

fn write_config(
    file_path: &str,
    config: &ical_config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let enc_config = ical_config::Config {
        ical: ical_config::Cal {
            ical_url: aes_cipher::aes_encrypt(&config.ical.ical_url),
            notice_timing: config.ical.notice_timing.clone(),
        },
    };

    let toml_string = toml::to_string(&enc_config)?;

    let mut file = File::create(file_path)?;

    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

pub fn update_config(config: &ical_config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir_path = paths::get_config_path();
    let config_file_path = config_dir_path.join("config.toml");

    write_config(config_file_path.to_str().unwrap(), &config)?;

    ical_config::set_config(&config);

    Ok(())
}

pub fn ensure_config_file() -> Result<bool, Box<dyn std::error::Error>> {
    let config_dir_path = paths::get_config_path();
    fs::create_dir_all(&config_dir_path)?;

    let config_file_path = config_dir_path.join("config.toml");

    let mut has_config = true;

    match read_config(config_file_path.to_str().unwrap()) {
        Ok(_) => {
            let log = format!("設定が正常にロードされました。");
            logger::logging_info(&log);
        }
        Err(_) => {
            logger::logging_error("設定ファイルが見つからないか、ロードに失敗しました。");

            let new_config = ical_config::Config {
                ical: ical_config::Cal {
                    ical_url: "".to_string(),
                    notice_timing: "20".to_string(),
                },
            };

            write_config(config_file_path.to_str().unwrap(), &new_config)?;

            ical_config::set_config(&new_config);

            let log = format!(
                "新しい設定ファイルを作成しました。 : {:?}",
                config_file_path
            );
            logger::logging_info(&log);

            has_config = false;
        }
    }

    Ok(has_config)
}
