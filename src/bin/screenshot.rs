use ashpd::desktop::screenshot::Screenshot;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let response = Screenshot::request()
        .interactive(true)
        .modal(true)
        .send()
        .await
        .unwrap()
        .response()?;
    println!("URI: {}", response.uri());
    Ok(())
}
