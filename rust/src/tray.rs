use crate::config::{Config, Mode};
use tray_icon::menu::{
    CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem,
};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct TrayHandle {
    pub tray: TrayIcon,
    pub toggle: CheckMenuItem,
    pub mode_item: MenuItem,
    pub autostart: CheckMenuItem,
}

impl TrayHandle {
    pub fn set_toggle_checked(&self, checked: bool) {
        self.toggle.set_checked(checked);
        let icon = if checked { sun_icon(255, 183, 77) } else { sun_icon(158, 158, 158) };
        let _ = self.tray.set_icon(Some(icon));
        let _ = self.tray.set_tooltip(Some(if checked {
            tr!("Keep Awake — Active", "保持唤醒 — 运行中")
        } else {
            tr!("Keep Awake — Stopped", "保持唤醒 — 已停止")
        }));
    }

    pub fn set_mode_label(&self, mode: &Mode) {
        self.mode_item.set_text(match mode {
            Mode::Api => tr!("Switch to Mouse Jiggle", "切换到鼠标微动"),
            Mode::Mouse => tr!("Switch to API Inhibit", "切换到API抑制"),
        });
    }

    pub fn set_autostart_checked(&self, checked: bool) {
        self.autostart.set_checked(checked);
    }

    pub fn show_notification(&self, title: &str, body: &str) {
        #[cfg(windows)]
        {
            use windows_sys::Win32::UI::Shell;
            use windows_sys::Win32::UI::WindowsAndMessaging::FindWindowW;
            unsafe {
                let hwnd = FindWindowW(windows_sys::core::w!("tray_icon_app"), std::ptr::null());
                if hwnd.is_null() {
                    return;
                }
                let mut nid: Shell::NOTIFYICONDATAW = std::mem::zeroed();
                nid.cbSize = std::mem::size_of::<Shell::NOTIFYICONDATAW>() as u32;
                nid.hWnd = hwnd;
                nid.uID = 1;
                nid.uFlags = Shell::NIF_INFO;
                let title16: Vec<u16> = title.encode_utf16().chain([0]).collect();
                let body16: Vec<u16> = body.encode_utf16().chain([0]).collect();
                let mut i = 0;
                for &c in &title16 { nid.szInfoTitle[i] = c; i += 1; if i >= 63 { break; } }
                i = 0;
                for &c in &body16 { nid.szInfo[i] = c; i += 1; if i >= 255 { break; } }
                Shell::Shell_NotifyIconW(Shell::NIM_MODIFY, &mut nid);
            }
        }
        #[cfg(not(windows))]
        let _ = (title, body);
    }
}

pub fn setup(config: &Config) -> (TrayHandle, MenuId, MenuId, MenuId, MenuId) {
    let menu = Menu::new();

    let toggle = CheckMenuItem::new(tr!("Keep Awake", "保持唤醒"), true, config.enabled, None);
    menu.append(&toggle).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let mode_label = match config.mode {
        Mode::Api => tr!("Switch to Mouse Jiggle", "切换到鼠标微动"),
        Mode::Mouse => tr!("Switch to API Inhibit", "切换到API抑制"),
    };
    let mode_item = MenuItem::new(mode_label, true, None);
    menu.append(&mode_item).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let autostart = CheckMenuItem::new(tr!("Launch at Login", "开机启动"), true, config.autostart, None);
    menu.append(&autostart).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let quit = MenuItem::new(tr!("Quit", "退出"), true, None);
    menu.append(&quit).ok();

    let icon = if config.enabled { sun_icon(255, 183, 77) } else { sun_icon(158, 158, 158) };

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip(if config.enabled {
            tr!("Keep Awake — Active", "保持唤醒 — 运行中")
        } else {
            tr!("Keep Awake — Stopped", "保持唤醒 — 已停止")
        })
        .build()
        .expect("Failed to create tray icon");

    let toggle_id = toggle.id().clone();
    let mode_id = mode_item.id().clone();
    let autostart_id = autostart.id().clone();
    let quit_id = quit.id().clone();
    let handle = TrayHandle { tray, toggle, mode_item, autostart };

    (handle, toggle_id, mode_id, autostart_id, quit_id)
}

fn sun_icon(r: u8, g: u8, b: u8) -> Icon {
    let size = 32u32;
    let cx = 15.5;
    let cy = 15.5;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    let rays: [(f64, f64); 8] = [
        (1.0, 0.0), (0.707, 0.707), (0.0, 1.0), (-0.707, 0.707),
        (-1.0, 0.0), (-0.707, -0.707), (0.0, -1.0), (0.707, -0.707),
    ];

    for py in 0..size {
        for px in 0..size {
            let dx = px as f64 - cx;
            let dy = py as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = ((py * size + px) * 4) as usize;

            // Sun body: filled circle radius 8
            if dist <= 8.0 {
                let edge = dist > 7.0;
                let alpha = if edge { ((8.0 - dist).max(0.0) * 255.0) as u8 } else { 255 };
                rgba[idx] = r;
                rgba[idx + 1] = g;
                rgba[idx + 2] = b;
                rgba[idx + 3] = alpha;
                continue;
            }

            // Rays: between radius 9 and 13
            if dist > 9.0 && dist <= 13.0 {
                for (rx, ry) in &rays {
                    let perp = (dx * ry - dy * rx).abs();
                    let proj = dx * rx + dy * ry;
                    if perp <= 1.8 && proj >= 9.0 && proj <= 13.0 {
                        let edge = perp > 1.0;
                        let alpha = if edge { ((1.8 - perp).max(0.0) * 255.0) as u8 } else { 255 };
                        rgba[idx] = r;
                        rgba[idx + 1] = g;
                        rgba[idx + 2] = b;
                        rgba[idx + 3] = alpha;
                        break;
                    }
                }
            }
        }
    }

    Icon::from_rgba(rgba, size, size).expect("Failed to create icon")
}
