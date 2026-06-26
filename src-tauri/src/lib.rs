#[tauri::command]
async fn launch_bombom(user: String, pwd: String, server_id: String) -> Result<String, String> {
    // Bu komut zaten main.rs içinde tanımlı, burayı boş geçebiliriz.
    Ok("Ok".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    // Hata veren eklenti satırını buradan tamamen kaldırdık!
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
