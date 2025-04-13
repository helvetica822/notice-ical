pub fn convert_timing_2_minutes(notice_timing: &str) -> i64 {
    let second = match notice_timing {
        "0" => 1,
        "10" => 3,
        "20" => 5,
        "30" => 10,
        "40" => 15,
        "50" => 30,
        "60" => 60,
        _ => 1,
    } * 60;

    second
}

pub fn check_timing_value(notice_timing: &str) -> String {
    let value = match notice_timing {
        "0" | "10" | "20" | "30" | "40" | "50" | "60" => notice_timing,
        _ => "20",
    };

    value.to_string()
}
