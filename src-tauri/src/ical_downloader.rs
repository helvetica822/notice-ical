use reqwest::{Client, Response};

pub async fn fetch_ical(url: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await.map_err(|e| {
        format!(
            "指定された URL から iCalendar を取得できませんでした。: {}",
            e
        )
    })?;

    if !response.status().is_success() {
        return Err(format!(
            "指定された URL から iCalendar を取得できませんでした。: {}",
            response.status()
        )
        .into());
    }

    let content_type = get_content_type(&response);

    if !content_type.contains("text/calendar") && !content_type.contains("application/ics") {
        return Err("指定された URL は iCalendar ではありません。".into());
    }

    let body = response.text().await?;

    Ok(body)
}

fn get_content_type(response: &Response) -> String {
    response
        .headers()
        .get("Content-Type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or_default()
        .to_string()
}
