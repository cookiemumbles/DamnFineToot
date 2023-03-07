use mastodon_async::prelude::Notification;

use crate::{
    read_data_from_json,
    string_utils::parsing::{extract_command_from_toot, CustomCommands},
};

use super::parsing::extract_url_from_toot;

#[test]
fn should_get_command_from_noti() {
    // given
    let notification: Notification = read_data_from_json("src/test_res/noti_perfect.json");
    let content = notification.status.unwrap().content;

    // when
    let result = extract_url_from_toot(&content).unwrap();

    // then
    assert_eq!(
        "https://ohai.social/@cookie_mumbles/109704675480017007",
        result.full_url
    );
    assert_eq!("@cookie_mumbles@ohai.social", result.user_handle);
}

#[test]
fn should_return_err_when_no_url_found() {
    // given
    let notification: Notification = read_data_from_json("src/test_res/noti_status.json");
    let content = notification.status.unwrap().content;

    // when
    let result = extract_url_from_toot(&content);

    // then
    assert!(result.is_err());
}

#[test]
fn should_get_command_from_content() {
    // given
    let notification: Notification = read_data_from_json("src/test_res/noti_status.json");
    let content = notification.status.unwrap().content;

    // when
    let result = extract_command_from_toot(&content);

    // then
    assert_eq!(CustomCommands::STATUS, result.unwrap());
    // assert!(result.is_err());
}
