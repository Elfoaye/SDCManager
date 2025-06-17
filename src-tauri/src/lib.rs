mod files_setup;
mod database;
mod settings;
mod admin_auth;
use database::{get_materiel_data, get_item_data, update_dispo, 
    update_item, add_item, delete_item, save_devis, load_devis};
use settings::{get_materiel_types, get_loc_formulas, 
    set_materiel_types, set_loc_formulas};
use admin_auth::{update_admin_password, log_in_admin, log_out_admin, is_admin};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler!
            [get_materiel_data, get_item_data,
            update_dispo, update_item, add_item, delete_item, 
            save_devis, load_devis,
            get_materiel_types, get_loc_formulas, 
            set_materiel_types, set_loc_formulas,
            update_admin_password, log_in_admin, log_out_admin, is_admin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
