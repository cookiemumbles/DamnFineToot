use super::parsing::extract_url_from_toot;

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
    let result = extract_url_from_toot(content).unwrap();
    assert_eq!(
        "https://ohai.social/@cookie_mumbles/109704675480017007",
        result.full_url
    );
    assert_eq!("@cookie_mumbles@ohai.social", result.user_handle);
}
