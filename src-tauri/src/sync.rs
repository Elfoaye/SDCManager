use crate::files_setup::get_or_create_data_dir;
use tauri::Manager;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use tokio::time::{sleep, Duration, Instant};
use ureq::{get, post};
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static SYNCTHING_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

const API_URL: &str = "http://localhost:8384";

fn get_api_key(path: &Path) -> Result<String, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let doc = roxmltree::Document::parse(&content).map_err(|e| e.to_string())?;

    let gui = doc.descendants().find(|n| n.has_tag_name("gui"))
        .ok_or("No <gui> tag found")?;
    let apikey_node = gui.children()
        .find(|n| n.has_tag_name("apikey"))
        .ok_or("Balise <apikey> introuvable")?;
    let apikey = apikey_node.text()
        .ok_or("Clé API vide")?
        .to_string();

    Ok(apikey)
}

async fn wait_for_config_file(path: &Path, timeout_secs: u64) -> Result<(), String> {
    let start = Instant::now();
    let deadline = start + Duration::from_secs(timeout_secs);
    
    while Instant::now() < deadline {
        if path.exists() {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(format!("Fichier {:?} non trouvé après {} secondes", path, timeout_secs))
}

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
        .arg("-no-browser")
        .arg("-home")
        .arg(&config_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Erreur de lancement Syncthing : {}", e))?;

    *SYNCTHING_PROCESS.lock().unwrap() = Some(child);

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

async fn wait_for_syncthing_api(api_key: &str) -> Result<(), String> {
    let start = Instant::now();
    let deadline = start + Duration::from_secs(10);

    while Instant::now() < deadline {
        let response = ureq::get(&format!("{}/rest/config", API_URL))
            .header("X-API-Key", api_key)
            .call();

        match response {
            Ok(resp) if resp.status() == 200 => {
                return Ok(());
            }
            Err(err) => {
                println!("Ping Syncthing échoué : {:?}", err);
            }
            _ => {
                println!("Ping Syncthing sans succès...");
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
    Err("Syncthing API non disponible après 10 secondes".into())
}

fn restart_syncthing(api_key: &str) -> Result<(), String> {
    let url = format!("{}/rest/system/restart", API_URL);
    let response = post(&url)
        .header("X-API-Key", api_key)
        .send("");

    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Erreur redémarrage Syncthing : {}", e)),
    }
}

fn get_local_device_id(api_key: &str) -> Result<String, String> {
    let url = format!("{}/rest/system/status", API_URL);
    let mut response = get(&url)
        .header("X-API-Key", api_key)
        .call()
        .map_err(|e| format!("Erreur lecture status Syncthing : {}", e))?;

    let status_json: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| format!("Erreur parsing status JSON : {}", e))?;

    let my_id = status_json["myID"]
        .as_str()
        .ok_or("Clé 'myID' manquante ou invalide")?;

    Ok(my_id.to_string())
}

fn configure_syncthing_folder(path: &Path, folder_id: &str, api_key: &str, my_id: &str) -> Result<(), String> {
    let url = format!("{}/rest/config", API_URL);

    let mut response = get(&url)
        .header("X-API-Key", api_key)
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
        "devices": [{ "deviceID": my_id }],
        "rescanIntervalS": 60,
        "fsWatcherEnabled": true,
        "fsWatcherDelayS": 10,
        "minDiskFreePct": 1.0,
        "versioning": {
            "type": "",
            "params": {}
        },
        "copiers": 1,
        "pullers": 1,
        "hashers": 0,
        "order": "random",
        "ignorePerms": true,
        "autoNormalize": true,
        "paused": false,
        "weakHashThresholdPct": 25,
        "markerName": ".stfolder"
    });

    let folders = config["folders"].as_array_mut()
        .ok_or("Configuration folders non trouvée ou invalide")?;

    let exists = folders.iter().any(|f| f["id"] == folder_id);
    if !exists {
        folders.push(folder_obj);
    } else {
        return Ok(());
    }

    ureq::put(&url)
        .header("X-API-Key", api_key)
        .send_json(&config)
        .map_err(|e| format!("Erreur mise à jour config Syncthing : {}", e))?;

    Ok(())
}

fn is_folder_configured(folder_id: &str, api_key: &str) -> Result<bool, String> {
    let url = format!("{}/rest/config", API_URL);

    let mut response = get(&url)
        .header("X-API-Key", api_key)
        .call()
        .map_err(|e| format!("Erreur lecture config Syncthing : {}", e))?;

    let config: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| format!("Erreur parsing config JSON: {}", e))?;

    if let Some(folders) = config.get("folders").and_then(|f| f.as_array()) {
        Ok(folders.iter().any(|f| f.get("id") == Some(&serde_json::Value::String(folder_id.to_string()))))
    } else {
        Ok(false)
    }
}

fn get_config_path(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(handle
        .path()
        .resolve("syncthing_config", tauri::path::BaseDirectory::AppData)
        .map_err(|e| e.to_string())?
        .join("config.xml"))
}

fn clear_syncthing_indexes(config_path: &Path) -> Result<(), String> {
    let index_dir = config_path
        .parent()
        .ok_or("Impossible de trouver le dossier de config Syncthing")?;

    for entry in fs::read_dir(index_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |f| f.to_string_lossy().starts_with("index-")) {
            fs::remove_file(&path).map_err(|e| format!("Erreur suppression index {:?} : {}", path, e))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_user_id(handle: tauri::AppHandle) -> Result<String, String> {
    let config_path = get_config_path(&handle).map_err(|e| e.to_string())?;

    wait_for_config_file(&config_path, 10).await?;
    let api_key = get_api_key(&config_path)?;

    let id = get_local_device_id(&api_key).map_err(|e| format!("Erreur lors de l'obtetion de l'id local: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub async fn add_user_to_sync_to(id: String, name: String, handle: tauri::AppHandle) -> Result<(), String> {
    let config_path = get_config_path(&handle).map_err(|e| e.to_string())?;

    wait_for_config_file(&config_path, 10).await?;
    let api_key = get_api_key(&config_path)?;

    let url = format!("{}/rest/config", API_URL);
    let mut response = ureq::get(&url)
        .header("X-API-Key", &api_key)
        .call()
        .map_err(|e| format!("Erreur lecture config : {}", e))?;

    let mut config: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| format!("Erreur parsing JSON : {}", e))?;

    let devices = config["devices"]
        .as_array_mut()
        .ok_or("Champ 'devices' introuvable")?;

    let is_first_device = devices.is_empty();

    if is_first_device {
        // Delete local data
        let data_dir = get_or_create_data_dir(&handle)?;
        if data_dir.exists() {
            for entry in fs::read_dir(&data_dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();

                if path.is_file() {
                    fs::remove_file(&path).map_err(|e| e.to_string())?;
                } 
            }
        }

        // Forcing sync
        clear_syncthing_indexes(&config_path)?;
    }

    if !devices.iter().any(|d| d["deviceID"] == id) {
        devices.push(json!({
            "deviceID": id,
            "name": name,
            "addresses": ["dynamic"],
            "compression": "metadata",
            "introducer": false,
            "skipIntroductionRemovals": false,
            "introducedBy": "",
            "paused": false,
            "allowedNetworks": [],
            "autoAcceptFolders": false,
            "maxSendKbps": 0,
            "maxRecvKbps": 0,
            "maxRequestKiB": 0,
            "untrusted": false,
            "remoteGUIPort": 0
        }));
    }

    let folders = config["folders"]
        .as_array_mut()
        .ok_or("Champ 'folders' introuvable")?;

    for folder in folders {
        if folder["id"] == "sdc-data" {
            let folder_devices = folder["devices"]
                .as_array_mut()
                .ok_or("Champ 'devices' du dossier invalide")?;
            
            if !folder_devices.iter().any(|d| d["deviceID"] == id) {
                folder_devices.push(json!({ "deviceID": id }));
            }
        }
    }

    // Appliquer la nouvelle config
    ureq::put(&url)
        .header("X-API-Key", &api_key)
        .send_json(&config)
        .map_err(|e| format!("Erreur mise à jour config : {}", e))?;

    restart_syncthing(&api_key)?;

    Ok(())
}

#[tauri::command]
pub async fn setup_syncthing_sync(handle: tauri::AppHandle) -> Result<String, String> {
    let data_dir = get_or_create_data_dir(&handle)?;

    launch_syncthing(&handle)?;

    let config_path = get_config_path(&handle).map_err(|e| e.to_string())?;

    wait_for_config_file(&config_path, 10).await?;
    let api_key = get_api_key(&config_path)?;

    wait_for_syncthing_api(&api_key).await?;

    let id = get_local_device_id(&api_key).map_err(|e| format!("Erreur lors de l'obtetion de l'id local: {}", e))?;


    if !is_folder_configured("sdc-data", &api_key)? {
        configure_syncthing_folder(&data_dir, "sdc-data", &api_key, &id)?;

        restart_syncthing(&api_key)?;
    } 
    Ok(id)
}