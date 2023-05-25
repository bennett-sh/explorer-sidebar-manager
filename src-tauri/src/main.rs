#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::ErrorKind, process::Command, thread};

use tauri::Manager;
use window_shadows::set_shadow;
use winreg::{enums::*, RegKey};

const YES: u32 = 0x1;

struct Shortcut {
    name: String,
    icon_path: String,
    icon_index: i32,
    path: String,
    guid: String,
}

impl serde::Serialize for Shortcut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Shortcut", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("iconPath", &self.icon_path)?;
        state.serialize_field("iconIndex", &self.icon_index)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("guid", &self.guid)?;
        state.end()
    }
}

#[tauri::command]
fn restart_explorer() {
    thread::spawn(|| {
        let taskkill_output = Command::new("taskkill")
            .arg("/f")
            .arg("/im")
            .arg("explorer.exe")
            .output();
        match taskkill_output {
            Ok(_) => {
                let start_output = Command::new("explorer.exe").output();
                match start_output {
                    Ok(_) => println!("explorer was restarted."),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    });
}

#[tauri::command]
fn add_shortcut(
    name: &str,
    path: &str,
    icon_path: &str,
    icon_index: &str,
    guid: &str,
) -> (bool, String) {
    let guid_formatted = format!("{{{}}}", guid.to_uppercase());
    let icon_index_parsed = match icon_index.parse::<i32>() {
        Ok(n) => n,
        Err(err) => return (false, err.to_string()),
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let clsid = match hkcu.open_subkey_with_flags(r#"Software\Classes\CLSID"#, KEY_ALL_ACCESS) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    let clsid_wow6432_node = match hkcu
        .open_subkey_with_flags(r#"Software\Classes\Wow6432Node\CLSID"#, KEY_ALL_ACCESS)
    {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };

    let clsid_sc = create_shortcut_in_clsid(
        clsid,
        name,
        path,
        icon_path,
        icon_index_parsed,
        &guid_formatted,
    );
    let clsid_wow6432_node_sc = create_shortcut_in_clsid(
        clsid_wow6432_node,
        name,
        path,
        icon_path,
        icon_index_parsed,
        &guid_formatted,
    );

    if !clsid_sc.0 {
        return clsid_sc;
    }
    if !clsid_wow6432_node_sc.0 {
        return clsid_wow6432_node_sc;
    }

    let explorer = match hkcu.open_subkey_with_flags(
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer"#,
        KEY_ALL_ACCESS,
    ) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };

    let hide_desktop_icons = match explorer
        .open_subkey_with_flags(r#"HideDesktopIcons\NewStartPanel"#, KEY_ALL_ACCESS)
    {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    match hide_desktop_icons.set_value(&guid_formatted, &YES) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    };

    let namespace = match explorer.open_subkey_with_flags(r#"Desktop\NameSpace"#, KEY_ALL_ACCESS) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    let (namespace_sub, _namespace_sub_disp) = match namespace.create_subkey(&guid_formatted) {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match namespace_sub.set_value("", &name) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    };

    (true, "".to_string())
}

fn create_shortcut_in_clsid(
    clsid: RegKey,
    name: &str,
    path: &str,
    icon_path: &str,
    icon_index: i32,
    guid_formatted: &String,
) -> (bool, String) {
    let prop_bag_attributes_value: u32 = 0x11;
    let folder_value_flags_value: u32 = 0x28;
    let attributes_value: u32 = 0xf080004d;
    let sort_order_index: u32 = 0x42;

    let (shortcut, _shortcut_disp) = match clsid.create_subkey(guid_formatted) {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match shortcut.set_value("", &name) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    match shortcut.set_value("SortOrderIndex", &sort_order_index) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    match shortcut.set_value("System.IsPinnedToNamespaceTree", &YES) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    if icon_path.len() > 0 {
        let (default_icon, _default_icon_disp) = match shortcut.create_subkey("DefaultIcon") {
            Ok(k) => k,
            Err(err) => return (false, err.to_string()),
        };
        match default_icon.set_value("", &format!("{},{}", icon_path, icon_index).as_str()) {
            Ok(()) => (),
            Err(err) => return (false, err.to_string()),
        }
    }
    let (inproc_server, _inproc_server_disp) = match shortcut.create_subkey("InProcServer32") {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match inproc_server.set_value("", &r#"%systemroot%\system32\shell32.dll"#) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    let (shell_folder, _shell_folder_disp) = match shortcut.create_subkey("ShellFolder") {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match shell_folder.set_value("Attributes", &attributes_value) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    match shell_folder.set_value("FolderValueFlags", &folder_value_flags_value) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    let (instance, _instance_disp) = match shortcut.create_subkey("Instance") {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match instance.set_value("CLSID", &"{0E5AAE11-A475-4c5b-AB00-C66DE400274E}") {
        Ok(()) => (),
        Err(err) => {
            return (false, err.to_string())
        }
    }
    let (init_prop_bag, _init_prop_bag_disp) = match instance.create_subkey("InitPropertyBag") {
        Ok(k) => k,
        Err(err) => return (false, err.to_string()),
    };
    match init_prop_bag.set_value("Attributes", &prop_bag_attributes_value) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    match init_prop_bag.set_value("TargetFolderPath", &path) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    }
    (true, "".to_string())
}

#[tauri::command]
fn cleanup_shortcut(id: &str) -> (bool, String) {
    let guid_formatted = format!("{{{}}}", id.to_uppercase());
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let clsid = match hkcu.open_subkey("Software\\Classes\\CLSID") {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    match clsid.delete_subkey_all(&guid_formatted) {
        Ok(()) => (),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return (true, "".to_string()),
            _ => return (false, err.to_string()),
        },
    };
    let explorer = match hkcu.open_subkey_with_flags(
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer"#,
        KEY_ALL_ACCESS,
    ) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };

    let hide_desktop_icons = match explorer
        .open_subkey_with_flags(r#"HideDesktopIcons\NewStartPanel"#, KEY_ALL_ACCESS)
    {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    match hide_desktop_icons.delete_value(&guid_formatted) {
        Ok(()) => (),
        Err(err) => return (false, err.to_string()),
    };

    let namespace = match explorer.open_subkey_with_flags(r#"Desktop\NameSpace"#, KEY_ALL_ACCESS) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string()),
    };
    match namespace.delete_subkey_all(guid_formatted) {
        Ok(()) => (),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return (true, "".to_string()),
            _ => return (false, err.to_string()),
        },
    };
    (true, "".to_string())
}

#[tauri::command]
fn get_shortcut_by_id(id: &str) -> (bool, String, Option<Shortcut>) {
    let guid_formatted = format!("{{{}}}", id.to_uppercase());
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let clsid = match hkcu.open_subkey("Software\\Classes\\CLSID") {
        Ok(key) => key,
        Err(err) => return (false, err.to_string(), None),
    };
    let shortcut = match clsid.open_subkey(guid_formatted) {
        Ok(key) => key,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return (false, "key doesn't exist anymore".to_string(), None),
            _ => return (false, err.to_string(), None),
        },
    };
    let name: String = match shortcut.get_value("") {
        Ok(name) => name,
        Err(err) => return (false, err.to_string(), None),
    };
    let bag = match shortcut.open_subkey(r#"Instance\InitPropertyBag"#) {
        Ok(key) => key,
        Err(err) => return (false, err.to_string(), None),
    };
    let path: String = match bag.get_value("TargetFolderPath") {
        Ok(path) => path,
        Err(err) => return (false, err.to_string(), None),
    };
    let no_icon_shortcut = Some(Shortcut {
        guid: id.to_string(),
        name: name.clone(),
        path: path.clone(),
        icon_index: 0,
        icon_path: "".to_string(),
    });
    let default_icon_key = match shortcut.open_subkey("DefaultIcon") {
        Ok(key) => key,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return (true, "".to_string(), no_icon_shortcut),
            _ => return (true, err.to_string(), None),
        },
    };
    let icon_path_with_index: String = match default_icon_key.get_value("") {
        Ok(path) => path,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return (true, "".to_string(), no_icon_shortcut),
            _ => return (true, err.to_string(), None),
        },
    };
    let split_icon_path = icon_path_with_index.split(',').collect::<Vec<&str>>();
    let icon_path = split_icon_path[0].to_string();
    let icon_index_str = if split_icon_path.len() > 1 {
        split_icon_path[1]
    } else {
        "0"
    };
    let icon_index = match icon_index_str.parse::<i32>() {
        Ok(i) => i,
        Err(_) => return (true, "".to_string(), no_icon_shortcut),
    };
    (
        true,
        "".to_string(),
        Some(Shortcut {
            guid: id.to_string(),
            name,
            path,
            icon_path,
            icon_index,
        }),
    )
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            app.windows()
                .iter()
                .for_each(|window| set_shadow(&window.1, true).unwrap_or_default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_shortcut,
            restart_explorer,
            get_shortcut_by_id,
            cleanup_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
