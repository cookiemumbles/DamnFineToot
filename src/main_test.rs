use async_trait::async_trait;
use mastodon_async::Result;

use regex::Regex;

use crate::{handle_notification, masto::api_wrapper::MastoWrapper, read_data_from_json};

struct MastoWrapperStub;

#[async_trait]
impl MastoWrapper for MastoWrapperStub {
    async fn send_public_toot(&self, text: String) -> Result<String> {
        Ok(format!("Toot sent: {}", text))
    }
}

#[tokio::test]
async fn should_handle_notification() {
    let notification = read_data_from_json("src/test_res/noti_perfect.json");

    let masto_wrapper = &MastoWrapperStub;

    let result = handle_notification(&notification, masto_wrapper).await;
    assert!(result.is_ok());
    assert_string_matches("^Toot sent:.*", result.unwrap().as_str());
}

#[tokio::test]
async fn should_ignore_favorites() {
    let notification = read_data_from_json("src/test_res/noti_favorite.json");

    let masto_wrapper = &MastoWrapperStub;

    let result = handle_notification(&notification, masto_wrapper).await;
    assert!(result.is_ok());
    assert_eq!("", result.unwrap());
}

#[tokio::test]
async fn should_ignore_replies() {
    let notification = read_data_from_json("src/test_res/noti_reply.json");

    let masto_wrapper = &MastoWrapperStub;

    let result = handle_notification(&notification, masto_wrapper).await;
    assert!(result.is_ok());
    assert_eq!("", result.unwrap());
}

fn assert_string_matches(expected_match_string: &str, actual: &str) {
    let re = Regex::new(expected_match_string).unwrap();
    assert!(re.is_match(actual));
}
