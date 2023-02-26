use futures::TryStream;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::Path;

use futures_util::TryStreamExt;
use mastodon_async::prelude::{Event, Notification};
use mastodon_async::Result;

use crate::masto_connect::{get_masto_instance, MastoWrapper, MastoWrapperReal};
use crate::texts::format_dft_toot;

mod masto_connect;

#[cfg(test)]
mod main_test;

mod texts;
#[cfg(test)]
mod texts_test;

#[tokio::main]
async fn main() -> Result<()> {
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
                    return Ok(msg)
                },
                Err(err) => {
                    eprint!("Stream ended with Error - {:?}", err);
                    return Err(err)
                }
            }
        }
        Err(err) => {
            eprintln!("Err: Streaming user broke - {:?}", err);
            return Err(err)
        },
    }
}

async fn handle_user_stream(
    masto: &dyn MastoWrapper,
    stream: impl TryStream<Ok = Event, Error = mastodon_async::Error>,
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
        "Recieved: notification of type: {:?}",
        notification.notification_type
    );

    let content = notification.status.clone().unwrap().content;
    let url = extract_url_from_toot(content.as_str()).unwrap();

    return masto
        .award_dft(format_dft_toot(
            url.user_handle.as_str(),
            format!("@{}", notification.account.acct).as_str(),
            url.full_url.as_str(),
        ))
        .await;
}

fn extract_url_from_toot(content: &str) -> Result<TootUrl> {
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

#[derive(Debug)]
pub struct TootUrl {
    full_url: String,
    user_handle: String,
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
