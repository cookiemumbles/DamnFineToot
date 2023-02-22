use futures_util::TryStreamExt;
use mastodon_async::{
    helpers::{cli, toml},
    prelude::Event,
    Mastodon, Registration,
};
use mastodon_async::{Result, StatusBuilder, Visibility};
use rand::seq::SliceRandom;

#[cfg(test)]
mod main_test;

#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = get_masto_instance().await?;
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
                    // println!("  content: {:?}", notification);
                    println!(
                        "Recieved: notification of type: {:?}",
                        notification.notification_type
                    );

                    let content = notification.status.clone().unwrap().content;
                    let url = extract_url(content.as_str()).unwrap();

                    award_dft(format_dft_toot(
                        url.user_handle.as_str(),
                        format!("@{}", notification.account.acct).as_str(),
                        url.full_url.as_str(),
                    ))
                    .await;
                }
            }
            Ok(())
        })
        .await?;

    println!("{:#?}", you);
    Ok(())
}

async fn get_masto_instance() -> Result<Mastodon> {
    let read_file_result = toml::from_file("mastodon-data.toml");
    return match read_file_result {
        Ok(data) => Ok(Mastodon::from(data)),
        Err(_) => Ok(register().await?),
    };
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://techhub.social")
        .client_name("DamnFineToot")
        .scopes(mastodon_async::scopes::Scopes::all())
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
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

async fn award_dft(text: String) {
    let mastodon = get_masto_instance().await.unwrap();
    println!("Sending toot: {}", text);
    let status = StatusBuilder::new()
        .status(text)
        .visibility(Visibility::Public)
        .build()
        .unwrap();
    let _ = mastodon.new_status(status).await;
}

#[derive(Debug)]
pub struct TootUrl {
    full_url: String,
    user_handle: String,
}
