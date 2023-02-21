use regex::Regex;

use crate::{extract_url, format_dft_toot};

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

    assert_eq!("@cookie_mumbles@techhub.social", capture.get(1).map_or("", |m| m.as_str()), "full output:{}", format_result);
    assert_eq!("@cookie_mumbles@ohai.social", capture.get(2).map_or("", |m| m.as_str()), "full output:{}", format_result);
    assert_eq!("https://ohai.social/@cookie_mumbles/109704675480017007", capture_url.get(0).map_or("", |m| m.as_str()), "full output:{}", format_result);
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
