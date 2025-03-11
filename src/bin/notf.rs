use std::{thread, time};

use ashpd::desktop::{
    notification::{Action, Button, Notification, NotificationProxy, Priority},
    Icon,
};
use futures_util::StreamExt;
use zbus::zvariant::Value;

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
    // let action = proxy
    //     .receive_action_invoked()
    //     .await?
    //     .next()
    //     .await
    //     .expect("Stream exhausted");
    dbg!("hi");
    // match action.name() {
    //     "copy" => (),   // Copy something to clipboard
    //     "delete" => (), // Delete the file
    //     _ => (),
    // };

    // println!("{:#?}", action.id());
    // println!(
    //     "{:#?}",
    //     action.parameter().get(0).unwrap().downcast_ref::<u32>()
    // );

    // proxy.remove_notification(notification_id).await?;
    Ok(())
}
