use futures_util::TryStreamExt;
use mastodon_async::prelude::Account;
use mastodon_async::{Result, StatusBuilder, Visibility};
use mastodon_async::{
    helpers::{cli, toml},
    prelude::Event,
    Mastodon, Registration,
};


async fn get_masto_instance() -> Result<Mastodon> {
    let read_file_result = toml::from_file("mastodon-data.toml");
    return match read_file_result {
        Ok(data) => Ok(Mastodon::from(data)),
        Err(_) => Ok(register().await?),
    };
}

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
                    award_dft(&notification.account, url).await;
                }
            }
            Ok(())
        })
        .await?;

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
        user_handle: format!("{}@{}", captures.get(3).unwrap().as_str(), captures.get(2).unwrap().as_str()),
    });
}

async fn award_dft(acct: &Account, url: TootUrl) {
    let mastodon = get_masto_instance().await.unwrap();
    println!("{} was awarded a DamnFineToot award by @{} for toot: {}", url.user_handle, acct.acct, url.full_url);
    let status = StatusBuilder::new()
        .status(format!("{} was awarded a DamnFineToot award by @{} for toot: {}", url.user_handle, acct.acct, url.full_url))
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

#[cfg(test)]
mod tests {
    use crate::extract_url;


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
        assert_eq!("https://ohai.social/@cookie_mumbles/109704675480017007", result.full_url);
        assert_eq!("@cookie_mumbles@ohai.social", result.user_handle);
    }
}

                    /*
                       content: Notification {
                           id: NotificationId("3781347"),
                           notification_type: Mention,
                           created_at: 2023-02-19 20:12:36.598 +00:00:00,
                           account: Account {
                               acct: "cookie_mumbles@techhub.social",
                               avatar: "https://s3.masto.ai/cache/accounts/avatars/109/466/670/536/867/967/original/6e914fdc35f1deb9.png",
                               avatar_static: "https://s3.masto.ai/cache/accounts/avatars/109/466/670/536/867/967/original/6e914fdc35f1deb9.png",
                               created_at: 2022-12-06 0:00:00.0 +00:00:00,
                               display_name: "Cookie Codes",
                               followers_count: 64,
                               following_count: 95,
                               header: "https://s3.masto.ai/cache/accounts/headers/109/466/670/536/867/967/original/13c75c7efddd82eb.jpeg",
                               header_static: "https://s3.masto.ai/cache/accounts/headers/109/466/670/536/867/967/original/13c75c7efddd82eb.jpeg",
                               id: AccountId("109466670536867967"),
                               locked: false,
                               note: "<p>Software developer that makes jokes over at <span class=\"h-card\"><a href=\"https://ohai.social/@cookie_mumbles\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>cookie_mumbles</span></a></span> and jokes and chats about software and other stuff here.</p><p>\u{
                               2029
                               }\u{2029}Creater of <a href=\"https://justmytoots.com\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">justmytoots.com</span><span class=\"invisible\"></span></a>\u{2029}\u{2029}\u{2029}</p><p>Github: <a href=\"https://github.com/cookiemumbles\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">github.com/cookiemumbles</span><span class=\"invisible\"></span></a></p><p><a href=\"https://techhub.social/tags/JustMyToots\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>JustMyToots</span></a></p>",
                               statuses_count: 224,
                               url: "https://techhub.social/@cookie_mumbles",
                               username: "cookie_mumbles",
                               source: None,
                               moved: None,
                               fields: Some([MetadataField {
                                   name: "Main account",
                                   value: "<a href=\"https://ohai.social/@cookie_mumbles\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">ohai.social/@cookie_mumbles</span><span class=\"invisible\"></span></a>" 
                               },
                               MetadataField {
                                   name: "My Toots",
                                   value: "<a href=\"https://justmytoots.com/@cookie_mumbles@techhub.social\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"ellipsis\">justmytoots.com/@cookie_mumble</span><span class=\"invisible\">s@techhub.social</span></a>" 
                               }]),
                               bot: Some(false) 
                           },
                           status: Some(Status {
                               id: StatusId("109893226008612329"),
                               uri: "https://techhub.social/users/cookie_mumbles/statuses/109893218185026946",
                               url: Some("https://techhub.social/@cookie_mumbles/109893218185026946"),
                               account: Account {
                                   acct: "cookie_mumbles@techhub.social",
                                   avatar: "https://s3.masto.ai/cache/accounts/avatars/109/466/670/536/867/967/original/6e914fdc35f1deb9.png",
                                   avatar_static: "https://s3.masto.ai/cache/accounts/avatars/109/466/670/536/867/967/original/6e914fdc35f1deb9.png",
                                   created_at: 2022-12-06 0:00:00.0 +00:00:00,
                                   display_name: "Cookie Codes",
                                   followers_count: 64,
                                   following_count: 95,
                                   header: "https://s3.masto.ai/cache/accounts/headers/109/466/670/536/867/967/original/13c75c7efddd82eb.jpeg",
                                   header_static: "https://s3.masto.ai/cache/accounts/headers/109/466/670/536/867/967/original/13c75c7efddd82eb.jpeg",
                                   id: AccountId("109466670536867967"),
                                   locked: false,
                                   note: "<p>Software developer that makes jokes over at <span class=\"h-card\"><a href=\"https://ohai.social/@cookie_mumbles\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>cookie_mumbles</span></a></span> and jokes and chats about software and other stuff here.</p><p>\u{
                                   2029
                                   }\u{2029}Creater of <a href=\"https://justmytoots.com\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">justmytoots.com</span><span class=\"invisible\"></span></a>\u{2029}\u{2029}\u{2029}</p><p>Github: <a href=\"https://github.com/cookiemumbles\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">github.com/cookiemumbles</span><span class=\"invisible\"></span></a></p><p><a href=\"https://techhub.social/tags/JustMyToots\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>JustMyToots</span></a></p>",
                                   statuses_count: 224,
                                   url: "https://techhub.social/@cookie_mumbles",
                                   username: "cookie_mumbles",
                                   source: None,
                                   moved: None,
                                   fields: Some([MetadataField {
                                       name: "Main account",
                                       value: "<a href=\"https://ohai.social/@cookie_mumbles\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">ohai.social/@cookie_mumbles</span><span class=\"invisible\"></span></a>" 
                                   },
                                   MetadataField {
                                       name: "My Toots",
                                       value: "<a href=\"https://justmytoots.com/@cookie_mumbles@techhub.social\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"ellipsis\">justmytoots.com/@cookie_mumble</span><span class=\"invisible\">s@techhub.social</span></a>" 
                                   }]),
                                   bot: Some(false) 
                               },
                               in_reply_to_id: None,
                               in_reply_to_account_id: None,
                            reblog: None,
                            content: "<p><span class=\"h-card\"><a href=\"https://masto.ai/@damnfinetoot\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>damnfinetoot</span></a></span> </p><p><a href=\"https://ohai.social/@cookie_mumbles/109887827013146620\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"ellipsis\">ohai.social/@cookie_mumbles/10</span><span class=\"invisible\">9887827013146620</span></a></p>",
                            created_at: 2023-02-19 20:10:35.0 +00:00:00,
                            emojis: [],
                            replies_count: Some(0),
                            reblogs_count: 0,
                            favourites_count: 0,
                            reblogged: Some(false),
                            favourited: Some(false),
                            sensitive: false,
                            spoiler_text: "",
                            visibility: Direct,
                            media_attachments: [],
                            mentions: [Mention {
                                url: "https://masto.ai/@damnfinetoot",
                                username: "damnfinetoot",
                                acct: "damnfinetoot",
                                id: "109876238547767407" 
                            }],
                            tags: [],
                            card: None,
                            application: None,
                            language: Some("en"),
                            pinned: None 
    }) }
                     */
