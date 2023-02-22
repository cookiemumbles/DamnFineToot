use async_trait::async_trait;
use mastodon_async::helpers::cli;
use mastodon_async::{helpers::toml, Mastodon};
use mastodon_async::{Registration, Result, StatusBuilder, Visibility};

#[async_trait]
pub trait MastoWrapper {
    async fn get_masto_instance(&self) -> Result<Mastodon>;
    async fn register(&self) -> Result<Mastodon>;
    async fn award_dft(&self, text: String) -> Result<String>;
}

pub struct MastoWrapperReal {}

#[async_trait]
impl MastoWrapper for MastoWrapperReal {
    async fn get_masto_instance(&self) -> Result<Mastodon> {
        let read_file_result = toml::from_file("mastodon-data.toml");
        return match read_file_result {
            Ok(data) => Ok(Mastodon::from(data)),
            Err(_) => Ok(self.register().await?),
        };
    }

    async fn register(&self) -> Result<Mastodon> {
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

    async fn award_dft(&self, text: String) -> Result<String> {
        let mastodon = self.get_masto_instance().await.unwrap();
        println!("Sending toot: {}", text);
        let status = StatusBuilder::new()
            .status(&text)
            .visibility(Visibility::Public)
            .build()
            .unwrap();
        mastodon.new_status(status).await?;
        Ok(format!("Toot sent: {}", text))
    }
}
