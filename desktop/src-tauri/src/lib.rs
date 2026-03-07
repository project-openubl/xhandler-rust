mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::create::create_xml,
            commands::sign::sign_xml,
            commands::send::send_xml,
            commands::verify_ticket::verify_ticket,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
