mod database;
mod settings;
use database::{get_materiel_data, get_item_data, update_dispo, 
    update_item, add_item, delete_item};
use settings::{get_materiel_types, get_loc_formulas};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler!
            [get_materiel_data, get_item_data,
            update_dispo, update_item,
            add_item, delete_item,
            get_materiel_types, get_loc_formulas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
