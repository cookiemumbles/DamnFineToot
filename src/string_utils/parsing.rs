use regex::Regex;

#[derive(Debug)]
pub struct TootUrl {
    pub full_url: String,
    pub user_handle: String,
}

fn sanitized_content(content: &str) -> String {
    return Regex::new(r#"<[^>]*>"#)
        .unwrap()
        .replace_all(content, "")
        .to_string();
}

pub fn extract_url_from_toot(content: &str) -> Result<TootUrl, String> {
    let sanitized_content = sanitized_content(content);

    let url_regex = Regex::new(r#"(https?://[^<\s\\"]+)"#).unwrap();
    let toot_url_regex = Regex::new(r#"(https?://([^/]+)/(@[^/]+)/[^<\s][^\s"]+)"#).unwrap();
    let captures = url_regex
        .captures_iter(sanitized_content.as_str())
        .map(|it| it.get(1).unwrap())
        .filter(|it| it.as_str() != "https://techhub.social/@DamnFineToot")
        .flat_map(|it| {
            println!("capt:{:?}", it);
            toot_url_regex.captures(it.as_str()).take()
        })
        .last();
    println!("  content: {:?}", captures);

    return match captures {
        Some(capts) => Ok(TootUrl {
            full_url: capts.get(1).unwrap().as_str().to_string(),
            user_handle: format!(
                "{}@{}",
                capts.get(3).unwrap().as_str(),
                capts.get(2).unwrap().as_str()
            ),
        }),
        None => Err(format!("No toot url found in: {:?}", content)),
    };
}

#[derive(Debug, PartialEq)]
pub enum CustomCommands {
    STATUS,
}

pub fn extract_command_from_toot(content: &str) -> Result<CustomCommands, String> {
    let sanitized_content = sanitized_content(content);
    let trimmed_string = Regex::new(r#"@DamnFineToot"#)
        .unwrap()
        .replace_all(sanitized_content.as_str(), "")
        .to_string()
        .trim()
        .to_lowercase();

    if trimmed_string.as_str() == "status" {
        return Ok(CustomCommands::STATUS);
    } else {
        return Err(format!("Unrecognised command in: {}", sanitized_content));
    }
}
