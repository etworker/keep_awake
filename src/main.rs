mod config;
mod inhibit;
mod mouse;
mod tray;

use config::{Config, Mode};
use inhibit::InhibitGuard;
use mouse::MouseJiggler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_icon::menu::MenuEvent;
use tray_icon::TrayIconEvent;

fn main() {
    let config = Arc::new(Mutex::new(Config::load()));
    let enabled = Arc::new(AtomicBool::new(config.lock().unwrap().enabled));
    let mouse_mode = Arc::new(AtomicBool::new(
        matches!(config.lock().unwrap().mode, Mode::Mouse),
    ));

    let guard: Arc<Mutex<Option<InhibitGuard>>> = Arc::new(Mutex::new(None));
    let jiggler: Arc<Mutex<Option<MouseJiggler>>> = Arc::new(Mutex::new(None));

    let (handle, toggle_id, mode_id, autostart_id, quit_id) = {
        let cfg = config.lock().unwrap();
        tray::setup(&cfg)
    };

    {
        let cfg = config.lock().unwrap();
        if cfg.enabled {
            activate(&cfg, &guard, &jiggler, &enabled, &mouse_mode);
        }
    }

    let menu_rx = MenuEvent::receiver();
    let _tray_rx = TrayIconEvent::receiver();

    loop {
        while let Ok(event) = menu_rx.try_recv() {
            let id = event.id();
            if *id == quit_id {
                guard.lock().unwrap().take();
                jiggler.lock().unwrap().take();
                return;
            } else if *id == toggle_id {
                let mut cfg = config.lock().unwrap();
                let new = !cfg.enabled;
                cfg.enabled = new;
                cfg.save().ok();
                enabled.store(new, Ordering::Relaxed);
                if new {
                    activate(&cfg, &guard, &jiggler, &enabled, &mouse_mode);
                } else {
                    guard.lock().unwrap().take();
                    jiggler.lock().unwrap().take();
                }
                handle.set_toggle_checked(new);
            } else if *id == mode_id {
                let mut cfg = config.lock().unwrap();
                cfg.mode = match cfg.mode {
                    Mode::Api => Mode::Mouse,
                    Mode::Mouse => Mode::Api,
                };
                cfg.save().ok();
                mouse_mode.store(matches!(cfg.mode, Mode::Mouse), Ordering::Relaxed);
                handle.set_mode_label(&cfg.mode);
                if cfg.enabled {
                    guard.lock().unwrap().take();
                    jiggler.lock().unwrap().take();
                    activate(&cfg, &guard, &jiggler, &enabled, &mouse_mode);
                }
            } else if *id == autostart_id {
                let mut cfg = config.lock().unwrap();
                cfg.autostart = !cfg.autostart;
                cfg.save().ok();
                handle.set_autostart_checked(cfg.autostart);
                apply_autostart(cfg.autostart);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn activate(
    cfg: &Config,
    guard: &Arc<Mutex<Option<InhibitGuard>>>,
    jiggler: &Arc<Mutex<Option<MouseJiggler>>>,
    enabled: &Arc<AtomicBool>,
    _mouse_mode: &Arc<AtomicBool>,
) {
    match cfg.mode {
        Mode::Api => {
            *guard.lock().unwrap() = InhibitGuard::acquire();
        }
        Mode::Mouse => {
            let interval = cfg.interval_secs;
            *jiggler.lock().unwrap() = Some(MouseJiggler::start(interval, enabled.clone()));
        }
    }
}

fn apply_autostart(enabled: bool) {
    if enabled {
        if let Ok(exe) = std::env::current_exe() {
            auto_start::enable(&exe);
        }
    } else {
        auto_start::disable();
    }
}

mod auto_start {
    pub fn enable(exe: &std::path::Path) {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("reg")
                .args([
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "keep-awake",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &exe.display().to_string(),
                    "/f",
                ])
                .output();
        }
        #[cfg(target_os = "macos")]
        {
            let plist = format!(
                r#"<?xml version="1.0"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>com.keep-awake</string>
<key>ProgramArguments</key><array><string>{}</string></array>
<key>RunAtLoad</key><true/>
</dict></plist>"#,
                exe.display()
            );
            if let Some(path) = dirs::home_dir()
                .map(|p| p.join("Library/LaunchAgents/com.keep-awake.plist"))
            {
                let _ = std::fs::write(&path, plist);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let desktop = format!(
                "[Desktop Entry]\nType=Application\nName=Keep Awake\nExec={}\nX-GNOME-Autostart-enabled=true\n",
                exe.display()
            );
            if let Some(path) = dirs::config_dir()
                .map(|p| p.join("autostart/keep-awake.desktop"))
            {
                let _ = std::fs::write(&path, desktop);
            }
        }
    }

    pub fn disable() {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("reg")
                .args([
                    "delete",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "keep-awake",
                    "/f",
                ])
                .output();
        }
        #[cfg(target_os = "macos")]
        {
            if let Some(path) = dirs::home_dir()
                .map(|p| p.join("Library/LaunchAgents/com.keep-awake.plist"))
            {
                let _ = std::fs::remove_file(&path);
            }
        }
        #[cfg(target_os = "linux")]
        {
            if let Some(path) = dirs::config_dir()
                .map(|p| p.join("autostart/keep-awake.desktop"))
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
}
