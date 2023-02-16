use mastodon_async::{helpers::{toml, cli}, Mastodon, Registration, Visibility};
use mastodon_async::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let read_file_result = toml::from_file("mastodon-data.toml");
    let mastodon = if let Ok(data) = read_file_result {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;

    // StatusBuilder::new();
    let status = mastodon_async::StatusBuilder::new().status("Daaaaamn").visibility(Visibility::Public).build().unwrap();
    mastodon.new_status(status).await?;


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
