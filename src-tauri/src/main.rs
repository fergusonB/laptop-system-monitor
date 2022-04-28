#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod get_data;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![

      get_data::total_memory,
      get_data::used_memory,
      get_data::free_memory,
      
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}





