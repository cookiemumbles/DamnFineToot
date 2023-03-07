use futures::TryStream;
use mastodon_async::entities::notification::NotificationType;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::Path;

use futures_util::TryStreamExt;
use mastodon_async::prelude::{Event, Notification};
use mastodon_async::{Error, Result};

use crate::masto::api_wrapper::{get_masto_instance, MastoWrapper, MastoWrapperReal};
use crate::string_utils::dft_msg::format_dft_toot;
use crate::string_utils::parsing::{extract_command_from_toot, CustomCommands};
use string_utils::parsing::{extract_url_from_toot, TootUrl};

mod masto;
mod string_utils;

#[cfg(test)]
mod main_test;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("-- starting at {} --", chrono::Local::now());
    eprint!("Connecting to account...");
    let masto = &MastoWrapperReal {
        api: get_masto_instance().await?,
    };
    eprintln!("[Ok]");

    eprint!("Verifying credentials...");
    let _user_creds = masto.api.verify_credentials().await?;
    eprintln!("[Ok]");

    eprintln!("Requesting stream...");
    let stream_result = masto.api.stream_user().await;
    match stream_result {
        Ok(stream) => {
            eprintln!("Received stream. Listening to notifications...");
            let stream_result = handle_user_stream(masto, stream).await;
            match stream_result {
                Ok(msg) => {
                    eprint!("Stream ended with Ok({:?})", msg);
                    return Ok(msg);
                }
                Err(err) => {
                    eprint!("Stream ended with Error - {:?}", err);
                    return Err(err);
                }
            }
        }
        Err(err) => {
            eprintln!("Err: Streaming user broke - {:?}", err);
            return Err(err);
        }
    }
}

async fn handle_user_stream(
    masto: &dyn MastoWrapper,
    stream: impl TryStream<Ok = Event, Error = Error>,
) -> Result<()> {
    stream
        .try_for_each(|event| async move {
            match event {
                Event::Update(ref _status) => { /* .. */ }
                Event::Delete(ref _id) => { /* .. */ }
                Event::FiltersChanged => { /* .. */ }
                Event::Notification(ref notification) => {
                    let result = handle_notification(notification, masto).await;
                    match result {
                        Ok(msg) => eprint!("Successful - {:?}", msg),
                        Err(err) => eprint!("Error - {:?}", err),
                    }
                }
            }
            Ok(())
        })
        .await
}

async fn handle_notification(
    notification: &Notification,
    masto: &dyn MastoWrapper,
) -> Result<String> {
    eprintln!(
        "Recieved: notification of type: {:?} at - {}",
        notification.notification_type,
        chrono::Local::now()
    );

    // For debugging
    // write_data_to_json_file(notification, format!("noti_{}.json", notification.id));

    if notification.notification_type == NotificationType::Mention
        && notification
            .status
            .clone()
            .unwrap()
            .in_reply_to_id
            .is_none()
    {
        let content = notification.status.clone().unwrap().content;
        return match extract_url_from_toot(content.as_str()) {
            Ok(url) => send_dft(notification, masto, url).await,
            Err(_) => match extract_command_from_toot(content.as_str()) {
                Ok(CustomCommands::STATUS) => send_status_reply(notification, masto).await,
                Err(_) => Err(Error::Other(String::from(""))),
            },
        };
    }
    Ok("".to_string())
}

async fn send_dft(
    notification: &Notification,
    masto: &dyn MastoWrapper,
    url: TootUrl,
) -> Result<String> {
    let toot_text = format_dft_toot(
        url.user_handle.as_str(),
        format!("@{}", notification.account.acct).as_str(),
        url.full_url.as_str(),
    );
    masto.send_public_toot(toot_text).await
}

async fn send_status_reply(
    notification: &Notification,
    masto: &dyn MastoWrapper,
) -> Result<String> {
    let id = notification.status.clone().unwrap();
    masto
        .send_reply(
            id.id.to_string(),
            format!("@{}\nStill here", notification.account.acct),
        )
        .await
}

pub fn write_data_to_json_file<T, P: AsRef<Path>>(data: &T, path: P)
where
    T: ?Sized + Serialize,
{
    let json_text =
        serde_json::to_string_pretty(data).expect("Unable to covert data object to json");
    fs::write(path, json_text).expect("Unable to write string to file");
}

// TODO: This is not safe and should be able to return an Err
pub fn read_data_from_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T {
    let result = std::fs::read_to_string(path).unwrap();
    return serde_json::from_str(&result).unwrap();
}
