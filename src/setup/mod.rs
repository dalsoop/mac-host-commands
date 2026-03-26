use std::path::Path;
use std::process::Command;

use crate::common;

pub fn status() {
    println!("=== 의존성 상태 ===\n");

    // Homebrew
    let (has_brew, _) = common::run_cmd_quiet("which", &["brew"]);
    println!("[brew] {}", if has_brew { "✓ 설치됨" } else { "✗ 미설치" });

    // macFUSE
    let has_macfuse = Path::new("/Library/Filesystems/macfuse.fs").exists()
        || Path::new("/usr/local/lib/libfuse.dylib").exists()
        || Path::new("/opt/homebrew/lib/libfuse.dylib").exists();
    println!("[macFUSE] {}", if has_macfuse { "✓ 설치됨" } else { "✗ 미설치" });

    // sshfs
    let (has_sshfs, _) = common::run_cmd_quiet("which", &["sshfs"]);
    println!("[sshfs] {}", if has_sshfs { "✓ 설치됨" } else { "✗ 미설치" });

    // sshpass
    let (has_sshpass, _) = common::run_cmd_quiet("which", &["sshpass"]);
    println!("[sshpass] {}", if has_sshpass { "✓ 설치됨" } else { "✗ 미설치" });

    if !has_macfuse || !has_sshfs {
        println!("\n  [!] sshfs 마운트를 사용하려면: mac-host-commands setup install-sshfs");
    }
}

pub fn install_sshfs() {
    // brew 확인
    let (has_brew, _) = common::run_cmd_quiet("which", &["brew"]);
    if !has_brew {
        eprintln!("[setup] Homebrew가 필요합니다.");
        eprintln!("  /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
        std::process::exit(1);
    }

    // macFUSE
    let has_macfuse = Path::new("/Library/Filesystems/macfuse.fs").exists();
    if has_macfuse {
        println!("[setup] macFUSE 이미 설치됨");
    } else {
        println!("[setup] macFUSE 설치 중...");
        let ok = Command::new("brew")
            .args(["install", "--cask", "macfuse"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if ok {
            println!("[setup] macFUSE 설치 완료");
            println!("  [!] 재부팅이 필요합니다. 재부팅 후 sshfs를 설치하세요:");
            println!("      mac-host-commands setup install-sshfs");
            return;
        } else {
            eprintln!("[setup] macFUSE 설치 실패");
            std::process::exit(1);
        }
    }

    // sshfs (macFUSE가 있어야 설치 가능)
    let (has_sshfs, _) = common::run_cmd_quiet("which", &["sshfs"]);
    if has_sshfs {
        println!("[setup] sshfs 이미 설치됨");
    } else {
        println!("[setup] sshfs 설치 중...");

        // gromgit/fuse tap 추가
        let _ = Command::new("brew")
            .args(["tap", "gromgit/fuse"])
            .status();

        let ok = Command::new("brew")
            .args(["install", "gromgit/fuse/sshfs-mac"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if ok {
            println!("[setup] sshfs 설치 완료");
        } else {
            eprintln!("[setup] sshfs 설치 실패");
            std::process::exit(1);
        }
    }

    println!("\n[setup] sshfs 마운트 준비 완료!");
    println!("  mac-host-commands mount up proxmox");
}

pub fn install_sshpass() {
    let (has_sshpass, _) = common::run_cmd_quiet("which", &["sshpass"]);
    if has_sshpass {
        println!("[setup] sshpass 이미 설치됨");
        return;
    }

    println!("[setup] sshpass 설치 중...");

    let _ = Command::new("brew")
        .args(["tap", "hudochenkov/sshpass"])
        .status();

    let ok = Command::new("brew")
        .args(["install", "hudochenkov/sshpass/sshpass"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if ok {
        println!("[setup] sshpass 설치 완료");
    } else {
        eprintln!("[setup] sshpass 설치 실패");
        std::process::exit(1);
    }
}

pub fn bootstrap() {
    println!("=== Mac 호스트 부트스트랩 ===\n");

    // 1. sshpass
    println!("--- [1/3] sshpass ---");
    install_sshpass();

    // 2. macFUSE + sshfs
    println!("\n--- [2/3] macFUSE + sshfs ---");
    install_sshfs();

    // 3. 설정 초기화
    println!("\n--- [3/3] 설정 초기화 ---");
    crate::config::Config::init();

    println!("\n=== 부트스트랩 완료 ===");
}
