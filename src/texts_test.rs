use crate::texts::format_dft_toot;

use regex::Regex;

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

    assert_eq!(
        "@cookie_mumbles@techhub.social",
        capture.get(1).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
    assert_eq!(
        "@cookie_mumbles@ohai.social",
        capture.get(2).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
    assert_eq!(
        "https://ohai.social/@cookie_mumbles/109704675480017007",
        capture_url.get(0).map_or("", |m| m.as_str()),
        "full output:{}",
        format_result
    );
}
