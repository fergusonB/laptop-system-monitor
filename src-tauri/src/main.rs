#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sysinfo::{System, SystemExt};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() {
  let mut sys = System::new_all();

// First we update all information of our `System` struct.
sys.refresh_all();

  // print all system information
  println!("{:#?}", sys.processors());
}
