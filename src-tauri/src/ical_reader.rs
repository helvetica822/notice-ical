use crate::primitive;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Default)]
pub struct CalInfo {
    pub summary: String,
    pub description: String,
    pub date_start: String,
    pub date_end: String,
}

pub fn read_ical(ical_path: &str) -> Result<Vec<CalInfo>, String> {
    let file = File::open(ical_path)
        .map_err(|e| format!("iCalendar を開くことに失敗しました。: {}", e))?;
    let buf = BufReader::new(file);

    let reader = ical::PropertyParser::from_reader(buf);

    let mut v = Vec::<CalInfo>::new();
    let mut current: Option<CalInfo> = None;

    let now = primitive::get_current_datetime_jst();

    for line in reader {
        match line {
            Ok(property) => {
                let name = property.name.clone();
                let value = property.value.clone().unwrap_or_default();

                match name.as_str() {
                    "BEGIN" => {
                        current = Some(Default::default());
                    }
                    "SUMMARY" => {
                        if let Some(ref mut c) = current {
                            c.summary = value;
                        }
                    }
                    "DESCRIPTION" => {
                        if let Some(ref mut c) = current {
                            c.description = value;
                        }
                    }
                    "DTSTART" => {
                        if let Some(ref mut c) = current {
                            if !value.is_empty() && !primitive::is_date(&value) {
                                match primitive::parse_datetime(&value) {
                                    Ok(date) => {
                                        //debug
                                        //println!("Parsed date for '{}': {:?}", &value, date);

                                        let duration = primitive::calculate_duration(now, date);
                                        let s = duration.num_seconds();

                                        if s >= 0 && s <= 3600 {
                                            c.date_start = value;
                                        }
                                    }
                                    Err(e) => {
                                        return Err(format!(
                                            "iCalendar の読込に失敗しました。: {}",
                                            e
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    "DTEND" => {
                        if let Some(mut info) = current.take() {
                            if !value.is_empty() && !primitive::is_date(&value) {
                                info.date_end = value;
                            }

                            if !info.date_start.is_empty() {
                                v.push(info);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                return Err(format!("iCalendar の読込に失敗しました。: {}", e));
            }
        }
    }

    Ok(v)
}
