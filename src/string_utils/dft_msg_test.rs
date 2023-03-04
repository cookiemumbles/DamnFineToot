use super::dft_msg::format_dft_toot;

use regex::Regex;

#[test]
fn should_format_toot() {
    // given
    let giver = "@cookie_mumbles@techhub.social";
    let receiver = "@cookie_mumbles@ohai.social";
    let url = "https://ohai.social/@cookie_mumbles/109704675480017007";

    // when
    let format_result = format_dft_toot(receiver, giver, url);

    // capture
    let re = Regex::new(r#"[^@]+(@[^@]+@[^\s]+) [^@]+(@[^@]+@[^\s]+)"#).unwrap();
    let capture = re.captures(&format_result).unwrap();
    let url_re = Regex::new(r#"https://.*"#).unwrap();
    let capture_url = url_re.captures(&format_result).unwrap();
    let first_handle = capture.get(1).map_or("", |m| m.as_str());
    let second_handle = capture.get(2).map_or("", |m| m.as_str());

    // then
    if first_handle == giver {
        // handles can be flipped
        assert_eq!(giver, first_handle, "full output:{}", format_result);
        assert_eq!(receiver, second_handle, "full output:{}", format_result);
    } else {
        assert_eq!(receiver, first_handle, "full output:{}", format_result);
        assert_eq!(giver, second_handle, "full output:{}", format_result);
    }
    assert_eq!(
        url,
        capture_url.get(0).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
}
