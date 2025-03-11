use ashpd::desktop::settings::Settings;
use futures_util::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ashpd::Result<()> {
    let proxy = Settings::new().await?;

    let clock_format = proxy
        .read::<String>("org.gnome.desktop.interface", "clock-format")
        .await?;
    println!("{:#?}", clock_format);

    let settings = proxy.read_all(&["org.gnome.desktop.interface"]).await?;
    println!("{:#?}", settings);

    let setting = proxy
        .receive_setting_changed()
        .await?
        .next()
        .await
        .expect("Stream exhausted");
    println!("{}", setting.namespace());
    println!("{}", setting.key());
    println!("{:#?}", setting.value());

    Ok(())
}
