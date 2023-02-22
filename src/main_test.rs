use async_trait::async_trait;
use mastodon_async::{Mastodon, Result};

use regex::Regex;

use crate::{
    extract_url, format_dft_toot, handle_notification, masto_connect::MastoWrapper,
    read_data_from_json,
};

struct MastoWrapperStub;

#[async_trait]
impl MastoWrapper for MastoWrapperStub {
    async fn get_masto_instance(&self) -> Result<Mastodon> {
        todo!();
    }
    async fn register(&self) -> Result<Mastodon> {
        todo!();
    }
    // let mut award_dft_text = "";
    async fn award_dft(&self, text: String) -> Result<String> {
        Ok(format!("Toot sent: {}", text))
    }
}

#[tokio::test]
async fn should_handle_notification() {
    let notification = read_data_from_json("src/test_res/notification.json");

    let masto_wrapper = &MastoWrapperStub;

    let result = handle_notification(&notification, masto_wrapper).await;
    assert!(result.is_ok());
    assert_string_matches("^Toot sent:.*", result.unwrap().as_str());
}

#[test]
fn should_get_url_from_string() {
    let content = "
                   <p>
                       <span class=\"h-card\">
                           <a href=\"https://masto.ai/@damnfinetoot\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@
                               <span>damnfinetoot
                               </span>
                           </a>
                       </span> 
                   </p>
                   <p>
                       <a href=\"https://ohai.social/@cookie_mumbles/109704675480017007\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">
                           <span class=\"invisible\">https://</span>
                           <span class=\"ellipsis\">ohai.social/@cookie_mumbles/10</span>
                           <span class=\"invisible\">9704675480017007</span>
                       </a>
                   </p>
        ";
    let result = extract_url(content).unwrap();
    assert_eq!(
        "https://ohai.social/@cookie_mumbles/109704675480017007",
        result.full_url
    );
    assert_eq!("@cookie_mumbles@ohai.social", result.user_handle);
}

#[test]
fn should_format_toot() {
    let format_result = format_dft_toot(
        "@cookie_mumbles@ohai.social",
        "@cookie_mumbles@techhub.social",
        "https://ohai.social/@cookie_mumbles/109704675480017007",
    );

    let re = Regex::new(r#"[^@]+(@[^@]+@[^\s]+) [^@]+(@[^@]+@[^\s]+)"#).unwrap();
    let capture = re.captures(&format_result).unwrap();
    let url_re = Regex::new(r#"https://.*"#).unwrap();
    let capture_url = url_re.captures(&format_result).unwrap();

    assert_eq!(
        "@cookie_mumbles@techhub.social",
        capture.get(1).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
    assert_eq!(
        "@cookie_mumbles@ohai.social",
        capture.get(2).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
    assert_eq!(
        "https://ohai.social/@cookie_mumbles/109704675480017007",
        capture_url.get(0).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
}

fn assert_string_matches(expected_match_string: &str, actual: &str) {
    let re = Regex::new(expected_match_string).unwrap();
    assert!(re.is_match(actual));
}
