use mastodon_async::{helpers::{toml, cli}, Mastodon, Registration, Visibility, StatusBuilder, prelude::Event};
use mastodon_async::Result;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let read_file_result = toml::from_file("mastodon-data.toml");
    let mastodon = if let Ok(data) = read_file_result {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;

    // let status = StatusBuilder::new()
    //     .status("Daaaaamn")
    //     .visibility(Visibility::Public)
    //     .build()
    //     .unwrap();

    // let stream = mastodon.new_status(status).await?;


    // let notifications = mastodon.notifications().await?;
    // notifications.initial_items.iter()
    //     .for_each(|status| { println!("{status:?}") });

    // let dm_stream = mastodon.stream_direct().await?;
    // dm_stream
    //     .try_for_each(|event| async move {
    //         println!("{:#?}", event);
    //         Ok(())
    //     }).await?;


    let stream = mastodon.stream_user().await?;
    stream
        .try_for_each(|event| async move {
            // println!("{:#?}", event);
            match event {
                Event::Update(ref _status) => { /* .. */ },
                Event::Delete(ref _id) => { /* .. */ },
                Event::FiltersChanged => { /* .. */ },
                Event::Notification(ref notification) => {
                    println!("Recieved: notification of type: {:?}", notification.notification_type);
                    println!("  content: {:?}", notification.status.clone().unwrap().content);
                },
            }
            Ok(())
        }).await?;
    //         notification_type: Mention,
    // Status.content


    println!("{:#?}", you);
    Ok(())
}


async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://masto.ai")
                                    .client_name("damnfinetoot")
                                    .scopes(mastodon_async::scopes::Scopes::all())
                                    .build()
                                    .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}
