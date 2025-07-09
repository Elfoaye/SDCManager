mod admin_auth;
mod database_doc;
mod database_items;
mod files_setup;
mod settings;
mod sync;
use admin_auth::{is_admin, log_in_admin, log_out_admin, update_admin_password};
use database_doc::{
    delete_devis, delete_facture, duplicate_devis, facture_from_devis, get_client_infos,
    get_devis_summaries, get_factures_summaries, load_devis, load_facture, load_devis_materiel, save_devis,
};
use database_items::{
    add_item, delete_item, get_item_data, get_item_dispo, get_materiel_data, update_item, get_factures_from_item,
};
use settings::{get_loc_formulas, get_materiel_types, set_loc_formulas, set_materiel_types};
use sync::{setup_syncthing_sync, stop_syncthing};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_materiel_data,
            get_item_data,
            update_item,
            get_factures_from_item,
            add_item,
            delete_item,
            get_item_dispo,
            save_devis,
            load_devis,
            load_facture,
            load_devis_materiel,
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
            is_admin,
            setup_syncthing_sync,
            stop_syncthing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
