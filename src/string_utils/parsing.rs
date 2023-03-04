#[derive(Debug)]
pub struct TootUrl {
    pub full_url: String,
    pub user_handle: String,
}

pub fn extract_url_from_toot(content: &str) -> std::io::Result<TootUrl> {
    let re = regex::Regex::new(r#"(https?://([^/]+)/(@[^/]+)/[^<\s][^\s"]+)"#).unwrap();
    let captures = re
        .captures_iter(content)
        .filter(|it| it.get(0).unwrap().as_str() != "https://masto.ai/@damnfinetoot")
        .last()
        .unwrap();
    println!("  content: {:?}", captures);

    return Ok(TootUrl {
        full_url: captures.get(0).unwrap().as_str().to_string(),
        user_handle: format!(
            "{}@{}",
            captures.get(3).unwrap().as_str(),
            captures.get(2).unwrap().as_str()
        ),
    });
}
