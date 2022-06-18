#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  AppHandle,
  SystemTray,
  SystemTrayEvent,
  CustomMenuItem,
  SystemTrayMenu,
  Icon,
};
use tauri::async_runtime::block_on;

fn tray_icon() ->Icon {
  Icon::Raw(include_bytes!("../icons/icon.png").to_vec())
}

#[cfg(target_os = "macos")]
fn tray_icon_loading() ->Vec<Icon> {
  let mut icon_vec: Vec<Icon> = Vec::new();
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_0.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_1.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_2.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_3.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_4.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_5.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_6.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_7.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_8.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_9.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_10.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_11.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_12.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_13.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_14.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_15.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_16.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_17.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_18.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_19.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_20.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_21.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_22.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_23.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_24.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_25.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_26.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_27.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_28.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/macos/loading_29.png").to_vec()));
  icon_vec
}


#[cfg(target_os = "windows")]
pub fn tray_icon_loading() ->Vec<Icon> {
  let mut icon_vec: Vec<Icon> = Vec::new();
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_0.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_1.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_2.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_3.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_4.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_5.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_6.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_7.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_8.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_9.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_10.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_11.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_12.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_13.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_14.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_15.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_16.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_17.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_18.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_19.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_20.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_21.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_22.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_23.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_24.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_25.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_26.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_27.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_28.ico").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/windows/loading_29.ico").to_vec()));
  icon_vec
}

fn tray() -> SystemTray {
  let animate_tray_icon = CustomMenuItem::new("animate_tray_icon", "animate tray icon");
  let tray_menu = SystemTrayMenu::new()
    .add_item(animate_tray_icon);
  let tray = SystemTray::new().with_menu(tray_menu);
  return tray;
}

fn tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // println!("{:?}", id);
        match id.as_str() {
          "animate_tray_icon" => {
            block_on(set_tray_icon(app.clone())).unwrap();
          },
          _ => {}
        }
      }
      _ => {}
    }
}


#[tauri::command]
async fn set_tray_icon(handle: AppHandle) -> Result<(), String> {
  // loop interval ms
  let ms = 50;
  let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(ms));
  let icon_vec = tray_icon_loading();
  tokio::spawn(async move {
    let mut i = 0;
    let handle = handle.tray_handle();
    loop {
      // Wait until next tick.
      intv.tick().await;
      #[cfg(target_os = "macos")]
      handle.set_icon_as_template(false).unwrap();
      handle.set_icon(icon_vec[i].clone()).unwrap();
      i = if i >= 29 { 0 } else { i+1 };
      // force break for test
      if i >= 29 {
        #[cfg(target_os = "macos")]
        handle.set_icon_as_template(true).unwrap();
        handle.set_icon(tray_icon()).unwrap();
        break;
      }
    }
  });
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .system_tray(tray())
    .on_system_tray_event(tray_event)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
