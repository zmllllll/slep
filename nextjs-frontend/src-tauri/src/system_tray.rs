use super::*;

pub(crate) struct SystemTray {
    pub(crate) handle: tauri::SystemTrayHandle,
}

pub(crate) enum SystemTrayEvent {
    TurnOnFlashing,
    TurnOffFlashing,
}

pub(super) async fn handle(
    mut system_tray_rx: tokio_stream::wrappers::UnboundedReceiverStream<SystemTrayEvent>,
    window: tauri::Window,
) -> Result<()> {
    use tokio_stream::StreamExt as _;
    let mut rgba = Vec::new();
    for i in 1..=128 * 128 {
        rgba.append(&mut vec![0, 0, 0, 1])
    }

    let icon = tauri::Icon::Raw(include_bytes!("../icons/test.ico").to_vec());

    let system_tray = crate::SYSTEM_TRAY
        .get()
        .ok_or(error::Error::System("system tray not exist".to_string()))?;
    let handle = &system_tray.handle;
    while let Some(event) = system_tray_rx.next().await {
        match event {
            SystemTrayEvent::TurnOnFlashing => loop {
                handle.set_icon(icon.clone()).unwrap();
                tracing::info!("light");
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                handle
                    .set_icon(tauri::Icon::Rgba {
                        rgba: rgba.clone(),
                        width: 128,
                        height: 128,
                    })
                    .unwrap();
                tracing::info!("dark");
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                // if !system_tray.flashing {
                //     break;
                // }
            },
            SystemTrayEvent::TurnOffFlashing => todo!(),
        }
    }
    Ok(())
}
