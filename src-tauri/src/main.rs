#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::copy_bidirectional;

#[derive(Deserialize)]
struct ServerResponse {
    error: i32,
    front_url: Option<String>,
}

#[derive(Serialize)]
struct CommandResult {
    success: bool,
    message: String,
}

// 5840 portuna gelen yerel trafiği uzak sunucuya tünelleyen asenkron proxy fonksiyonu
async fn start_proxy(remote_target: String) {
    let listener = match TcpListener::bind("127.0.0.1:5840").await {
        Ok(l) => l,
        Err(_) => return, // Port zaten kullanımdaysa veya hata oluştuysa çık
    };
    println!("[proxy] Yerel 5840 portu dinleniyor, hedef: {}", remote_target);

    tokio::spawn(async move {
        while let Ok((mut local_stream, _)) = listener.accept().await {
            let target = remote_target.clone();
            tokio::spawn(async move {
                if let Ok(mut remote_stream) = TcpStream::connect(target).await {
                    // Gelen ve giden paketleri el değmeden çift yönlü köprüle
                    let _ = copy_bidirectional(&mut local_stream, &mut remote_stream).await;
                }
            });
        }
    });
}

#[tauri::command]
async fn launch_bombom(user: String, pwd: String, server_id: String) -> Result<CommandResult, String> {
    println!("[*] 1. Adım: API'ye giriş yapılıyor...");

    let client = reqwest::Client::builder()
    .cookie_store(true)
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
    .build()
    .map_err(|e| format!("İstemci başlatılamadı: {}", e))?;

    // 1. ADIM: Login POST İsteği
    let login_url = "https://www.337.com/api.php?lang=tr";
    let login_form = [
        ("a", "1002"),
        ("username", &user),
        ("password", &pwd),
        ("lang", "tr"),
    ];

    let login_res = client.post(login_url)
    .header("Accept", "application/json, text/javascript, */*; q=0.01")
    .header("Origin", "https://www.337.com")
    .header("Referer", "https://www.337.com/")
    .form(&login_form)
    .send()
    .await
    .map_err(|e| format!("Giriş bağlantı hatası: {}", e))?;

    let login_text = login_res.text().await.unwrap_or_default();
    if !login_text.contains("ok") {
        return Ok(CommandResult { success: false, message: "Kullanıcı adı veya şifre hatalı!".into() });
    }
    println!("[+] Giriş başarılı!");

    // 2. ADIM: Sunucu Bilgisi Çekme
    println!("[*] 2. Adım: Sunucu {} bağlantısı alınıyor...", server_id);
    let server_url = format!("https://www.337.com/api.php?a=1024&serverId={}&lang=tr&is_client=1", server_id);

    let server_res = client.get(&server_url)
    .header("Accept", "application/json, text/javascript, */*; q=0.01")
    .header("X-Requested-With", "XMLHttpRequest")
    .header("Referer", "https://www.337.com/pages/50000/bombom")
    .send()
    .await
    .map_err(|e| format!("Sunucu bağlantı hatası: {}", e))?;

    let json_data: ServerResponse = server_res.json()
    .await
    .map_err(|e| format!("Sunucu yanıtı çözümlenemedi: {}", e))?;

    if json_data.error != 0 {
        return Ok(CommandResult {
            success: false,
            message: format!("Sunucu Hatası (Kod {}): Giriş başarısız.", json_data.error)
        });
    }

    let mut front_url = json_data.front_url.unwrap_or_default();
    if front_url.starts_with("roadclient://") {
        front_url = front_url.replace("roadclient://", "");
    }

    // 3. ADIM: Nihai SWF ve Uzak Sunucu IP Çözümleme
    println!("[*] 3. Adım: Oyun linki çözümleniyor...");
    let auth_res = client.get(&front_url)
    .header("Referer", "https://www.337.com/")
    .send()
    .await
    .map_err(|e| format!("Oyun linki doğrulama hatası: {}", e))?;

    let final_url = auth_res.url().to_string();
    let mut swf_url = final_url.replace("Default.aspx", "Loading.swf");

    // Uzak sunucunun adresini (IP/Domain) URL'den ayıklıyoruz
    let remote_host = auth_res.url().host_str().unwrap_or("s1.bombom.337.com").to_string();
    // Bombom oyun sunucuları genelde soket için 7000-8000 arası veya direkt web portunu kullanır,
    // Biz tüneli resmi sunucunun default soket portuna (genelde 5840 veya web soketine) yönlendiriyoruz.
    let remote_target = format!("{}:5840", remote_host);

    // DİKKAT: Arka planda sahte yerel sunucumuzu (Proxy) tetikliyoruz!
    start_proxy(remote_target).await;

    let base_url = if swf_url.contains("Loading.swf") {
        swf_url.split("Loading.swf").next().unwrap_or("")
    } else {
        ""
    };

    if !swf_url.contains("config=") && !base_url.is_empty() {
        swf_url = format!("{}&config={}config.xml", swf_url, base_url);
    }

    // Ruffle Emülatörünü Eski Kararlı Ayarlarla Başlatma
    println!("[game] Ruffle başlatılıyor...");
    let mut cmd = Command::new("ruffle");
    cmd.env_remove("WAYLAND_DISPLAY")
    .env("GDK_BACKEND", "x11")
    .env("WINIT_UNIX_BACKEND", "x11");

    cmd.arg("--scale").arg("show-all")
    .arg("--force-scale")
    .arg("--quality").arg("best")
    .arg("--no-gui")
    .arg("--tcp-connections").arg("allow")
    .arg("--socket-allow").arg("127.0.0.1")
    .arg(&swf_url);

    match cmd.spawn() {
        Ok(_) => Ok(CommandResult { success: true, message: "Oyun başarıyla başlatıldı!".into() }),
        Err(_) => Ok(CommandResult { success: false, message: "Sistemde 'ruffle' komutu bulunamadı!".into() }),
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![launch_bombom])
    .run(tauri::generate_context!())
    .expect("tauri uygulaması çalıştırılırken hata oluştu");
}
