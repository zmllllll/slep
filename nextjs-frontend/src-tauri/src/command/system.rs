use instant::Duration;

use crate::SYSTEM_TRAY;

use super::*;

#[tauri::command]
pub(crate) async fn system_tray_flash() -> Result<(), Error> {
    let system_tray = SYSTEM_TRAY
        .get()
        .ok_or(Error::System("system tray not exist".to_string()))?;
    let icon = tauri::Icon::Raw(include_bytes!("../../icons/test.ico").to_vec());

    let mut rgba = Vec::new();
    for i in 1..=128 * 128 {
        rgba.append(&mut vec![0, 0, 0, 1])
    }

    loop {
        system_tray.handle.set_icon(icon.clone()).unwrap();
        tracing::info!("light");
        tokio::time::sleep(Duration::from_millis(1000)).await;
        system_tray
            .handle
            .set_icon(tauri::Icon::Rgba {
                rgba: rgba.clone(),
                width: 128,
                height: 128,
            })
            .unwrap();
        tracing::info!("dark");
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
    Ok(())
}
