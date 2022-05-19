
use sysinfo::{System, SystemExt};

#[tauri::command]
pub fn main ()->String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();

    //return json of variables
    let json = format!("{{\"total_memory\":{},\"used_memory\":{},\"free_memory\":{}}}", total_memory, used_memory, free_memory);
    json.into()

}


