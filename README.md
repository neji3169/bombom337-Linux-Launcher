#  Bombom 337 - Özel Linux İstemcisi (Launcher)

Bombom 337 oynamak için Flash Player derdinden kurtaran, **Ruffle** entegreli ve kendi içinde asenkron TCP Soket Proxy barındıran modern Linux masaüstü istemcisidir. Tauri (Rust + JS) mimarisiyle geliştirilmiştir.

## ✨ Özellikler
- **Hesap Yönetimi (Beni Hatırla):** Girdiğiniz hesapları kaydeder, listeden tek tıkla otomatik doldurur ve oyuna giriş yapar .
- **Modern Arayüz:** Sisteminizin temasına göre otomatik olarak değişen (Koyu/Açık Mod uyumlu)   minimalist ve şık tasarım.
- **Tam Linux Desteği:** Wayland ve X11 ortamlarında yerleşik entegrasyonla sorunsuz çalışır.

## ⚠️ Ruffle Kurulumu ve Gereksinimler (Kritik)

Uygulamanın arka planda Flash tabanlı `.swf` dosyalarını açabilmesi için sisteminizde **Ruffle emülatörünün yüklü ve terminalden doğrudan çalıştırılabilir (`PATH` içinde) olması zorunludur.**

**Ruffle'ı sisteminize kurmak için şu adımları izleyin:**
1. [Ruffle Resmi github Reposundan] (https://github.com/ruffle-rs/ruffle) Linux mimarisine uygun versiyonu indirin (Örn: `tar.gz`).
2. İndirdiğiniz arşivi bir klasöre çıkartın.
3. Dosyaları çıkardığının klasörün içinde terminal açın.
4. İçinden çıkan `ruffle` dosyasını sistemin doğrudan görebileceği bir dizine taşıyın ve yetki verin:
   ```bash
   sudo chmod +x ruffle
   sudo mv ruffle /usr/local/bin/

   Test etmek için terminale ruffle --version yazdığınızda sürüm bilgisi geliyorsa kurulum başarılıdır.

Geliştirici Ortamı Gereksinimleri (Kaynak koddan derlemek için):

    Node.js & npm

    Rust (Cargo)

    Gerekli Tauri Linux sistem kütüphaneleri (Örn: webkit2gtk, libssl-dev vb.)

   🚀 Kurulum ve Çalıştırma

Projeyi bilgisayarınıza klonlayıp kullanmak veya geliştirmek için şu komutları sırasıyla çalıştırın:
# Repoyu bilgisayarınıza klonlayın
git clone [https://github.com/neji3169/bombom337-Linux-Launcher.git](https://github.com/neji3169/bombom337-Linux-Launcher.git)
cd bombom337-Linux-Launcher

# Gerekli önyüz bağımlılıklarını yükleyin
npm install

# Geliştirici modunda anında test etmek için
npm run tauri dev

# Kurulabilir .AppImage tek tık paketi oluşturmak için (Build)
npm run tauri build

🛠️ Mimari ve Nasıl Çalışıyor?

    Launcher, 337.com API'sine istek atarak başarılı bir giriş yapar ve o anki Flash sunucu URL'sini (front_url) dinamik olarak çözer.

    Ruffle, oyunu TCP izinleri (--socket-allow 127.0.0.1) aktifleştirilmiş halde başlatır.

    Oyun paket gönderdiğinde, uygulamamız bu trafiği havada yakalar ve resmi Bombom oyun sunucusuna şeffaf bir köprü (tokio::io::copy_bidirectional) vasıtasıyla aktarır.





























# Tauri + Vanilla


This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
