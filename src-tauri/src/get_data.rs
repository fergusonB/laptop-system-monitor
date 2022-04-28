
use sysinfo::{System, SystemExt};


// Memory
#[tauri::command]
pub fn total_memory() -> String{
  System::new_all().total_memory().to_string().into()
}
#[tauri::command]
pub fn used_memory() -> String{
  System::new_all().used_memory().to_string().into()
}
#[tauri::command]
pub fn free_memory() -> String{
  System::new_all().available_memory().to_string().into()
}
