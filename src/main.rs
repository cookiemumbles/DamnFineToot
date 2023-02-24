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
    "Damn fine indeed!",
    "Whoa!",
    "Whoa nelly!",
    "Wow!",
    "Unbelievable!",
    "Can you believe it?",
    "Isn't it lovely?",
    "Oh happy day!",
    "I couldn't be happier.",
    "I think it is the right choice.",
    "Great selection!",
    "Great choice!",
    "A fine pick!",
    "It's a great day!",
    "That toot tho.",
    "What that toot do?",
    "Amazing stuff!",
    "Great stuff!",
    "Such good words.",
    "How do you like that?",
    "Congratulations!",
    "Boy howdy!",
    "I cannot disagree.",
    "Well what do you know.",
    "Nice!",
    "Noice!",
    "Daaaaang!",
    "It was a long time coming!",
    "It's a good one folks.",
    "I mean, it's so true.",
    "Hey now!",
    "It is known.",
    "Best toot since sliced bread.",
    "This is even better than that other one.",
    "Ain't it a peach?",
    "You betcha!",
    "You never think it will happen to you.",
    "Better than Shakespeare.",
    "When it's good it's good.",
    "Ho. Lee. Shit.",
    "WHOAH!!!",
    "Shut the front door!",
    "I'm super excited!",
    "Stop the presses!",
    "Ding ding ding!",
    "We have a winner!",
    "Oooo it's one of my favorites.",
    "Egad!",
    "What a toot!",
    "This one was overdue.",
    "It's so fetch!",
    "Yessirree!",
    "It's a work of art.",
    "They like you. They really like you.",
    "Inconceivable!",
    "Without doubt excellent.",
    "Simply the best.",
    "Almost as good as Murder She Wrote.",
    "Huzzah!",
    "What's not to like?",
    "I laughed, I cried.",
    "Five stars, would read again.",
    "Booyah!",
    "One toot to rule them all.",
    "Let's celebrate!",
    "Stunning bit of prose.",
    "Significantly better than Ezra.",
    "Fancy that!",
    "Good toot or best ever?",
    "Dios mio!",
    "It is without peer.",
    "EEEEEEEE!",
    "Ohhhhh yeah!",
    "Yo Adrian!",
    "Like a fine wine.",
    "Valar dohaeris!",
    "A masterpiece.",
    "Makes me want to dance.",
    "Outstanding!",
    "Open the champagne!",
    "Oofda dat's a good one!",
    "Whoa Nelly, would you look at that!",
    "Feels so good!",
    "That's the stuff!",
    "Excelsior!",
    "Praise be!",
    "Hot damn!",
    "Brilliant!",
    "A work of genius.",
    "Cheers!",
    "So freaking cool.",
    "Just as I expected.",
    "Fantastico!",
    "Hip hip hurray!",
    "Well well well.",
    "The Duke's mayonnaise of tweets.",
    "Glorious!",
    "Tweetariffic!",
    "Wish I had thought of it.",
    "Clever!",
    "That's some toot!",
    "The quality we deserve.",
    "So choice!",
    "Gold star!",
    "A work of great literature, really.",
    "Truly something to behold.",
    "We could use more like this one.",
    "Like a ray of sunshine!",
    "Truly inspirational.",
    "On a scale of 1 to 10: 11.",
    "I am so proud.",
    "Such a refreshing perspective.",
    "Simply perfect.",
    "This is the way.",
    "I like good tweets and I cannot lie.",
    "I gotta read that one again!",
    "Never mind DFT, Pulitzer for this one!",
    "A special unicorn of a toot.",
    "A real gem.",
    "Clearly the best one today.",
    "Where has this toot been all my life?",
    "Swipe right!",
    "I tip my chapeau!",
    "Stand up and holla!",
    "Standing ovation!",
    "Take a victory lap!",
    "Goodness gracious!",
    "I don't say this to just anyone.",
    "Pretty, pretty, pretty, pretty good.",
    "Dude.",
    "Better than Improv Night!",
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
