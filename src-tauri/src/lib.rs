mod admin_auth;
mod database_items;
mod database_doc;
mod files_setup;
mod settings;
use admin_auth::{is_admin, log_in_admin, log_out_admin, update_admin_password};
use database_items::{add_item, delete_item, get_item_data, get_materiel_data, update_dispo, update_item,};
use database_doc::{delete_devis, delete_facture, duplicate_devis, facture_from_devis, get_client_infos, get_devis_summaries, get_factures_summaries,
load_devis, load_facture, save_devis,};
use settings::{get_loc_formulas, get_materiel_types, set_loc_formulas, set_materiel_types};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_materiel_data,
            get_item_data,
            update_dispo,
            update_item,
            add_item,
            delete_item,
            save_devis,
            load_devis,
            load_facture,
            delete_devis,
            delete_facture,
            duplicate_devis,
            facture_from_devis,
            get_devis_summaries,
            get_factures_summaries,
            get_client_infos,
            get_materiel_types,
            get_loc_formulas,
            set_materiel_types,
            set_loc_formulas,
            update_admin_password,
            log_in_admin,
            log_out_admin,
            is_admin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
