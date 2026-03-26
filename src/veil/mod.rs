use std::path::Path;
use std::process::Command;

use crate::common;

fn home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/Users/jeonghan".to_string())
}

const VAULTCENTER_LXC: &str = "110";
const VAULTCENTER_IP: &str = "10.50.0.110";
const VAULTCENTER_PORT: &str = "11181";
const LOCALVAULT_PORT: &str = "10180";

pub fn status() {
    println!("=== VeilKey 상태 ===\n");

    // veilkey-cli
    let (has_cli, _) = common::run_cmd_quiet("which", &["veilkey-cli"]);
    println!("[veilkey-cli] {}", if has_cli { "✓ 설치됨" } else { "✗ 미설치" });

    // veil alias
    let (has_veil, _) = common::run_cmd_quiet("which", &["veil"]);
    println!("[veil] {}", if has_veil { "✓ 설치됨" } else { "✗ 미설치" });

    // LocalVault
    let (_, lv_resp) = common::run_cmd_quiet("curl", &["-s", &format!("http://127.0.0.1:{LOCALVAULT_PORT}/health")]);
    let lv_ok = lv_resp.contains("ok");
    println!("[LocalVault] 127.0.0.1:{LOCALVAULT_PORT} {}", if lv_ok { "✓ 실행 중" } else { "✗ 미실행" });

    // veilkey-localvault 바이너리
    let (has_lv_bin, _) = common::run_cmd_quiet("which", &["veilkey-localvault"]);
    println!("[LocalVault 바이너리] {}", if has_lv_bin { "✓ 설치됨" } else { "✗ 미설치" });

    // LaunchAgent
    let plist = format!("{}/Library/LaunchAgents/com.veilkey.localvault.plist", home());
    let has_plist = Path::new(&plist).exists();
    println!("[LaunchAgent] {}", if has_plist { "✓ 등록됨 (부팅 시 자동 시작)" } else { "✗ 미등록" });

    // VaultCenter (원격)
    let cfg = crate::config::Config::load();
    let (vc_ok, _) = common::ssh_cmd(&cfg.proxmox.host, &cfg.proxmox.user,
        &format!("pct exec {VAULTCENTER_LXC} -- curl -sk https://localhost:{VAULTCENTER_PORT}/health 2>/dev/null"));
    println!("[VaultCenter] LXC {VAULTCENTER_LXC} ({VAULTCENTER_IP}:{VAULTCENTER_PORT}) {}",
        if vc_ok { "✓ 실행 중" } else { "✗ 미실행" });

    // SSH 터널 (VaultCenter 접근용)
    let (_, ps) = common::run_cmd_quiet("pgrep", &["-f", &format!("ssh.*{VAULTCENTER_PORT}.*{VAULTCENTER_IP}")]);
    let tunnel_exists = !ps.trim().is_empty();
    println!("[VaultCenter 터널] {}", if tunnel_exists { "✓ 연결됨" } else { "✗ 미연결" });

    // .veilkey.sh 설정
    let has_profile = Path::new(&format!("{}/.veilkey.sh", home())).exists();
    println!("[셸 프로필] {}", if has_profile { "✓ ~/.veilkey.sh" } else { "✗ 미설정" });
}

pub fn install_cli() {
    let (has_cli, _) = common::run_cmd_quiet("which", &["veilkey-cli"]);
    if has_cli {
        println!("[veil] veilkey-cli 이미 설치됨");
        return;
    }

    println!("[veil] veilkey-cli 설치 중...");

    // Proxmox에서 바이너리 가져오기
    let cfg = crate::config::Config::load();
    let local_bin = format!("{}/.local/bin", home());
    common::ensure_dir(Path::new(&local_bin));

    // LXC에서 빌드된 바이너리 확인
    let (ok, path) = common::ssh_cmd(&cfg.proxmox.host, &cfg.proxmox.user,
        "which veilkey-cli 2>/dev/null || find /opt/veilkey -name veilkey-cli -type f 2>/dev/null | head -1");

    if ok && !path.trim().is_empty() {
        let remote_path = path.trim();
        println!("[veil] Proxmox에서 바이너리 복사: {remote_path}");
        let (ok, _, _) = common::run_cmd("scp", &[
            &format!("{}@{}:{}", cfg.proxmox.user, cfg.proxmox.host, remote_path),
            &format!("{local_bin}/veilkey-cli"),
        ]);
        if ok {
            let _ = Command::new("chmod").args(["+x", &format!("{local_bin}/veilkey-cli")]).output();
            println!("[veil] veilkey-cli 설치 완료");
        }
    } else {
        eprintln!("[veil] Proxmox에서 veilkey-cli를 찾을 수 없습니다.");
        eprintln!("  수동 설치가 필요합니다.");
        std::process::exit(1);
    }
}

pub fn install_localvault() {
    let (has_lv, _) = common::run_cmd_quiet("which", &["veilkey-localvault"]);
    if has_lv {
        println!("[veil] veilkey-localvault 이미 설치됨");
    } else {
        println!("[veil] veilkey-localvault 설치 중...");

        let cfg = crate::config::Config::load();
        let (ok, path) = common::ssh_cmd(&cfg.proxmox.host, &cfg.proxmox.user,
            "which veilkey-localvault 2>/dev/null || find /opt/veilkey -name veilkey-localvault -type f 2>/dev/null | head -1");

        if ok && !path.trim().is_empty() {
            let remote_path = path.trim();
            let (ok, _, _) = common::run_cmd("scp", &[
                &format!("{}@{}:{}", cfg.proxmox.user, cfg.proxmox.host, remote_path),
                "/usr/local/bin/veilkey-localvault",
            ]);
            if ok {
                let _ = Command::new("chmod").args(["+x", "/usr/local/bin/veilkey-localvault"]).output();
                println!("[veil] veilkey-localvault 설치 완료");
            }
        } else {
            eprintln!("[veil] Proxmox에서 veilkey-localvault를 찾을 수 없습니다.");
            std::process::exit(1);
        }
    }

    // LaunchAgent 등록
    setup_launchagent();
}

fn setup_launchagent() {
    let plist_path = format!("{}/Library/LaunchAgents/com.veilkey.localvault.plist", home());
    if Path::new(&plist_path).exists() {
        println!("[veil] LaunchAgent 이미 등록됨");
        return;
    }

    println!("[veil] LaunchAgent 등록 중...");
    let plist = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.veilkey.localvault</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/veilkey-localvault</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{home}/Library/Logs/veilkey-localvault.log</string>
    <key>StandardErrorPath</key>
    <string>{home}/Library/Logs/veilkey-localvault.log</string>
</dict>
</plist>"#, home = home());

    std::fs::write(&plist_path, plist).expect("LaunchAgent plist 생성 실패");

    let _ = Command::new("launchctl")
        .args(["load", &plist_path])
        .status();

    println!("[veil] LaunchAgent 등록 완료 (부팅 시 자동 시작)");
}

pub fn connect_vaultcenter() {
    let cfg = crate::config::Config::load();

    // 기존 터널 확인
    let (_, ps) = common::run_cmd_quiet("pgrep", &["-f", &format!("ssh.*{VAULTCENTER_PORT}.*{VAULTCENTER_IP}")]);
    if !ps.trim().is_empty() {
        println!("[veil] VaultCenter 터널 이미 연결됨 (localhost:{VAULTCENTER_PORT})");
        return;
    }

    println!("[veil] VaultCenter SSH 터널 연결 중...");
    println!("  localhost:{VAULTCENTER_PORT} -> {VAULTCENTER_IP}:{VAULTCENTER_PORT} (via {})", cfg.proxmox.host);

    let ok = Command::new("ssh")
        .args(["-f", "-N",
            "-L", &format!("{VAULTCENTER_PORT}:{VAULTCENTER_IP}:{VAULTCENTER_PORT}"),
            &format!("{}@{}", cfg.proxmox.user, cfg.proxmox.host)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if ok {
        println!("[veil] VaultCenter 터널 연결 완료");
        println!("  https://localhost:{VAULTCENTER_PORT}");
    } else {
        eprintln!("[veil] VaultCenter 터널 연결 실패");
    }
}

pub fn disconnect_vaultcenter() {
    let (_, ps) = common::run_cmd_quiet("pgrep", &["-f", &format!("ssh.*{VAULTCENTER_PORT}.*{VAULTCENTER_IP}")]);
    if ps.trim().is_empty() {
        println!("[veil] VaultCenter 터널이 연결되어 있지 않습니다.");
        return;
    }

    let _ = Command::new("pkill")
        .args(["-f", &format!("ssh.*{VAULTCENTER_PORT}.*{VAULTCENTER_IP}")])
        .status();
    println!("[veil] VaultCenter 터널 해제 완료");
}

pub fn setup_profile() {
    let profile_path = format!("{}/.veilkey.sh", home());

    let content = format!(r#"# ── VeilKey Shell Profile ──────────────────────────────────────────
# mac-host-commands 에서 자동 생성

# 환경변수
export VEILKEY_LOCALVAULT_URL="http://127.0.0.1:{LOCALVAULT_PORT}"
export VEILKEY_VAULTCENTER_URL="https://localhost:{VAULTCENTER_PORT}"
export VEILKEY_API="${{VEILKEY_LOCALVAULT_URL}}"

# alias
alias vk='veilkey-cli'
"#);

    std::fs::write(&profile_path, content).expect(".veilkey.sh 생성 실패");
    println!("[veil] ~/.veilkey.sh 생성 완료");

    // .zshrc에 source 추가 확인
    let zshrc = format!("{}/.zshrc", home());
    let zshrc_content = std::fs::read_to_string(&zshrc).unwrap_or_default();
    if !zshrc_content.contains(".veilkey.sh") {
        println!("[veil] .zshrc에 아래 라인을 추가하세요:");
        println!("  source ~/.veilkey.sh");
    } else {
        println!("[veil] .zshrc에 이미 source ~/.veilkey.sh 포함됨");
    }
}

pub fn localvault_start() {
    let (_, resp) = common::run_cmd_quiet("curl", &["-s", &format!("http://127.0.0.1:{LOCALVAULT_PORT}/health")]);
    if resp.contains("ok") {
        println!("[veil] LocalVault 이미 실행 중");
        return;
    }

    println!("[veil] LocalVault 시작 중...");
    let plist = format!("{}/Library/LaunchAgents/com.veilkey.localvault.plist", home());
    if Path::new(&plist).exists() {
        let _ = Command::new("launchctl").args(["load", &plist]).status();
    } else {
        // 직접 실행
        let _ = Command::new("veilkey-localvault")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
    let (_, resp) = common::run_cmd_quiet("curl", &["-s", &format!("http://127.0.0.1:{LOCALVAULT_PORT}/health")]);
    if resp.contains("ok") {
        println!("[veil] LocalVault 시작 완료");
    } else {
        eprintln!("[veil] LocalVault 시작 실패");
    }
}

pub fn localvault_stop() {
    let plist = format!("{}/Library/LaunchAgents/com.veilkey.localvault.plist", home());
    if Path::new(&plist).exists() {
        let _ = Command::new("launchctl").args(["unload", &plist]).status();
    } else {
        let _ = Command::new("pkill").args(["-f", "veilkey-localvault"]).status();
    }
    println!("[veil] LocalVault 중지 완료");
}

pub fn bootstrap() {
    println!("=== VeilKey 부트스트랩 ===\n");

    println!("--- [1/4] veilkey-cli 설치 ---");
    install_cli();

    println!("\n--- [2/4] LocalVault 설치 ---");
    install_localvault();

    println!("\n--- [3/4] VaultCenter 연결 ---");
    connect_vaultcenter();

    println!("\n--- [4/4] 셸 프로필 설정 ---");
    setup_profile();

    println!("\n=== VeilKey 부트스트랩 완료 ===");
}
