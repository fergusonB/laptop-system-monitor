#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod get_data;

fn main() {


  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![

     get_data::main,
      
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}





