use std::path::Path;

use async_trait::async_trait;
use mastodon_async::helpers::cli;
use mastodon_async::{helpers::toml, Mastodon};
use mastodon_async::{Registration, Result, StatusBuilder, Visibility};

const DATA_FILE_PATH: &str = "/etc/damnfinetoot/mastodon-data.toml";
const DATA_FILE_PATH_LOCAL: &str = "mastodon-data.toml";

#[async_trait]
pub trait MastoWrapper {
    async fn send_public_toot(&self, text: String) -> Result<String>;
}

pub struct MastoWrapperReal {
    pub api: Mastodon,
}

#[async_trait]
impl MastoWrapper for MastoWrapperReal {
    async fn send_public_toot(&self, text: String) -> Result<String> {
        eprintln!("Sending toot: {}", text);
        let status = StatusBuilder::new()
            .status(&text)
            .visibility(Visibility::Public)
            .build()
            .unwrap();
        self.api.new_status(status).await?;
        Ok(format!("Toot sent: {}", text))
    }
}

pub async fn get_masto_instance() -> Result<Mastodon> {
    let data_file_path = match Path::new(DATA_FILE_PATH).try_exists().unwrap() {
        true => DATA_FILE_PATH,
        false => DATA_FILE_PATH_LOCAL,
    };
    let read_file_result = toml::from_file(data_file_path);
    return match read_file_result {
        Ok(data) => Ok(Mastodon::from(data)),
        Err(_) => Ok(register().await?),
    };
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://techhub.social")
        .client_name("DamnFineTootBot")
        .scopes(mastodon_async::scopes::Scopes::all())
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;
    // Save app data for using on the next run.
    eprintln!("writing to {}", DATA_FILE_PATH_LOCAL);
    toml::to_file(&mastodon.data, DATA_FILE_PATH_LOCAL)?;
    eprintln!("success");

    Ok(mastodon)
}
