use ashpd::desktop::notification::{Notification, NotificationProxy, Priority};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let proxy = NotificationProxy::new().await?;
    dbg!("hi");
    dbg!(
        proxy
            .add_notification(
                "test_id",
                Notification::new("title")
                    .body("color copied to clipboard")
                    .priority(Priority::High),
            )
            .await?
    );
    dbg!("hi");

    proxy.remove_notification("test_id").await?;
    Ok(())
}
