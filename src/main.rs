use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::Path;

use futures_util::TryStreamExt;
use mastodon_async::prelude::{Event, Notification};
use mastodon_async::Result;
use rand::seq::SliceRandom;

use crate::masto_connect::{MastoWrapper, MastoWrapperReal};

mod masto_connect;

#[cfg(test)]
mod main_test;

#[tokio::main]
async fn main() -> Result<()> {
    let masto_wrapper = &MastoWrapperReal {};
    let mastodon = masto_wrapper.get_masto_instance().await?;
    let you = mastodon.verify_credentials().await?;

    println!("Listening to notifications...");

    let stream = mastodon.stream_user().await?;
    stream
        .try_for_each(|event| async move {
            match event {
                Event::Update(ref _status) => { /* .. */ }
                Event::Delete(ref _id) => { /* .. */ }
                Event::FiltersChanged => { /* .. */ }
                Event::Notification(ref notification) => {
                    let _ = handle_notification(notification, masto_wrapper).await;
                }
            }
            Ok(())
        })
        .await?;

    println!("{:#?}", you);
    Ok(())
}

async fn handle_notification(
    notification: &Notification,
    masto_wrapper: &dyn MastoWrapper,
) -> Result<String> {
    println!(
        "Recieved: notification of type: {:?}",
        notification.notification_type
    );

    let content = notification.status.clone().unwrap().content;
    let url = extract_url(content.as_str()).unwrap();

    return masto_wrapper
        .award_dft(format_dft_toot(
            url.user_handle.as_str(),
            format!("@{}", notification.account.acct).as_str(),
            url.full_url.as_str(),
        ))
        .await;
}

fn extract_url(content: &str) -> Result<TootUrl> {
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

const STATEMENTS: &'static [&str] = &[
    "The quality we deserve.",
    "Truly something to behold.",
    "Wow!",
    "Isn't it lovely?",
    "Amazing stuff!",
    "Stop the presses!",
    "On a scale of 1 to 10: 11.",
    "A work of genius.",
    "Truly inspirational.",
    "Ohhhhh yeah!",
    "Oooo it's one of my favorites.",
    "WHOAH!!!",
    "Never mind DFT, Pulitzer for this one!",
    "Hey now!",
    "Inconceivable!",
    "Clever!",
    "Valar dohaeris!",
    "Whoa Nelly, would you look at that!",
    "I'm super excited!",
    "Ain't it a peach?",
    "Outstanding!",
    "What's not to like?",
    "Isn't it lovely?",
    "That toot tho.",
    "Can you believe it?",
    "Dude.",
    "Ain't it a peach?",
    "It's a great day!",
    "Great choice!",
    "Makes me want to dance.",
    "Without doubt excellent.",
    "Daaaaang!",
    "Whoa!",
    "Significantly better than Ezra.",
    "Five stars, would read again.",
    "Noice!",
];

fn format_dft_toot(receiver: &str, sender: &str, toot_url: &str) -> String {
    let handle_texts = [
        format!(
            "{} 's pick for toot of the day is by {} .",
            sender, receiver
        ),
        format!(
            "{} 's toot was selected by {} as the toot of the day.",
            receiver, sender
        ),
        format!("{} named {} 's toot the best of the day.", sender, receiver),
        format!("{} picked you, {} .", sender, receiver),
        format!("{} selected {} 's toot.", sender, receiver),
        format!("{} was trophied by {} .", receiver, sender),
    ];
    let selected_statements: Vec<&str> = STATEMENTS
        .choose_multiple(&mut rand::thread_rng(), 3)
        .cloned()
        .collect();
    return format!(
        "{} {} {} {}\n{}",
        selected_statements[0],
        selected_statements[1],
        handle_texts.choose(&mut rand::thread_rng()).unwrap(),
        selected_statements[2],
        toot_url
    );
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
