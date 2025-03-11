use std::{fs::File, os::fd::AsFd};

use ashpd::desktop::wallpaper::{SetOn, WallpaperRequest};

async fn run() -> ashpd::Result<()> {
    let file = File::open("/home/vrn21/Downloads/wallpaper.jpg").unwrap();
    WallpaperRequest::default()
        .set_on(SetOn::Both)
        .show_preview(true)
        .build_file(&file.as_fd())
        .await?;
    Ok(())
}

async fn run_uri() -> ashpd::Result<()> {
    let uri = url::Url::parse("file:///home/vrn21/Downloads/wallpaper.jpg").unwrap();
    WallpaperRequest::default()
        .set_on(SetOn::Both)
        .show_preview(true)
        .build_uri(&uri)
        .await?;
    Ok(())
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> ashpd::Result<()> {
    run().await?;
    // run_uri().await?;
    Ok(())
}
