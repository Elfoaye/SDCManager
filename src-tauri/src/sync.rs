use crate::files_setup::get_or_create_data_dir;
use tauri::Manager;
use std::fs;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use tokio::time::{sleep, Duration};
use ureq::{get, post};
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static SYNCTHING_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

const API_URL: &str = "http://localhost:8384";
const API_KEY: &str = "syncthing";

pub fn launch_syncthing(handle: &tauri::AppHandle) -> Result<(), String> {
    let os_name = std::env::consts::OS;
    let binary_name = if os_name == "windows" {
        "syncthing.exe"
    } else {
        "syncthing"
    };

    let relative_path = format!("bin/{}/{}", os_name, binary_name);

    let syncthing_path = handle
        .path()
        .resolve(relative_path, tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let config_dir = handle
        .path()
        .resolve("syncthing_config", tauri::path::BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    let child = Command::new(&syncthing_path)
        .arg("-home")
        .arg(&config_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Erreur de lancement Syncthing : {}", e))?;

    *SYNCTHING_PROCESS.lock().unwrap() = Some(child);

    println!("Syncthing lancé");

    Ok(())
}

#[cfg(windows)]
fn kill_process_tree(pid: u32) -> std::io::Result<()> {
    Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/T", "/F"])
        .status()?;
    Ok(())
}

#[tauri::command]
pub fn stop_syncthing() {
    if let Some(mut child) = SYNCTHING_PROCESS.lock().unwrap().take() {
        #[cfg(windows)]
        {
            let _ = kill_process_tree(child.id());
        }
        #[cfg(not(windows))]
        {
            let _ = child.kill();
        }
        let _ = child.wait();
    }
}

async fn wait_for_syncthing_api() -> Result<(), String> {
    println!("Début de l'attente de l'api...");
    for i in 0..30 {
        // match ureq::get(&format!("{}/rest/system/ping", API_URL))
        //     .header("X-API-Key", API_KEY)
        //     .call() {
        //     Ok(resp) if resp.status() == 200 => return Ok(()),
        //     _ => sleep(Duration::from_secs(1)).await,
        // }
        let result = ureq::get(&format!("{}/rest/system/ping", API_URL))
        .header("X-API-Key", API_KEY)
        .call();

        match result {
            Ok(resp) if resp.status() == 200 => {
                println!("[{}] Syncthing API disponible (code: 200)", i);
                return Ok(());
            },
            Ok(resp) => {
                println!("[{}] Réponse Syncthing : code {}", i, resp.status());
            },
            Err(e) => {
                println!("[{}] Erreur Syncthing : {}", i, e);
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
    Err("Syncthing API non disponible après 30 secondes".into())
}

fn restart_syncthing() -> Result<(), String> {
    println!("Redémarage de syncthing");
    let url = format!("{}/rest/system/restart", API_URL);
    let response = post(&url)
        .header("X-API-Key", API_KEY)
        .send("");

    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Erreur redémarrage Syncthing : {}", e)),
    }
}

fn configure_syncthing_folder(path: &Path, folder_id: &str) -> Result<(), String> {
    let url = format!("{}/rest/config", API_URL);

    let mut response = get(&url)
        .header("X-API-Key", API_KEY)
        .call()
        .map_err(|e| format!("Erreur lecture config Syncthing : {}", e))?;

    let mut config: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| format!("Erreur parsing config JSON: {}", e))?;

    let folder_obj = json!({
        "id": folder_id,
        "label": "SDC Data",
        "path": path.to_str().ok_or("Chemin invalide")?,
        "type": "sendreceive",
        "devices": ["self"],
        "rescanIntervalS": 60,
        "fsWatcherEnabled": true,
        "fsWatcherDelayS": 10
    });

    let folders = config["folders"].as_array_mut()
        .ok_or("Configuration folders non trouvée ou invalide")?;

    let exists = folders.iter().any(|f| f["id"] == folder_id);
    if !exists {
        folders.push(folder_obj);
    } else {
        return Ok(());
    }

    post(&url)
        .header("X-API-Key", API_KEY)
        .send_json(&config)
        .map_err(|e| format!("Erreur mise à jour config Syncthing : {}", e))?;
    println!("Syncthing configuré !");
    Ok(())
}

fn is_folder_configured(folder_id: &str) -> Result<bool, String> {
    println!("Verification de la configuration : ");
    let url = format!("{}/rest/config", API_URL);
    let mut response = get(&url)
        .header("X-API-Key", API_KEY)
        .call()
        .map_err(|e| format!("Erreur lecture config Syncthing : {}", e))?;

    let config: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| format!("Erreur parsing config JSON: {}", e))?;

    if let Some(folders) = config.get("folders").and_then(|f| f.as_array()) {
        println!("Configuration Ok");
        Ok(folders.iter().any(|f| f.get("id") == Some(&serde_json::Value::String(folder_id.to_string()))))
    } else {
        println!("Configuration à faire");
        Ok(false)
    }
}

#[tauri::command]
pub async fn setup_syncthing_sync(handle: tauri::AppHandle) -> Result<(), String> {
    println!("Début du setup de syncthing...");
    let data_dir = get_or_create_data_dir(&handle)?;

    launch_syncthing(&handle)?;

    wait_for_syncthing_api().await?;
    println!("Fin de l'attente de l'api");

    if !is_folder_configured("sdc-data")? {
        println!("Configuation de syncthing...");
        configure_syncthing_folder(&data_dir, "sdc-data")?;
        restart_syncthing()?;
    }

    Ok(())
}