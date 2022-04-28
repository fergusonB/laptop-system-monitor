
use sysinfo::{System, SystemExt};


// Memory
#[tauri::command]
pub fn total_memory() -> String{
  System::new_all().total_memory().to_string().into()
}
