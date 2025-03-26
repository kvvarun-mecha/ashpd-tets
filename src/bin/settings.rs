use ashpd::desktop::settings::Settings;
use futures_util::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ashpd::Result<()> {
    let proxy = Settings::new().await?;
    let test = proxy.read_all(&["org.freedesktop.appearance"]).await;
    println!("{:?}", test);
    Ok(())
}
