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
        let _ = self.tray.set_tooltip(Some(if checked {
            "Keep Awake — Enabled"
        } else {
            "Keep Awake — Disabled"
        }));
    }

    pub fn set_mode_label(&self, mode: &Mode) {
        self.mode_item.set_text(match mode {
            Mode::Api => "Switch to Mouse Jiggle",
            Mode::Mouse => "Switch to API Inhibit",
        });
    }

    pub fn set_autostart_checked(&self, checked: bool) {
        self.autostart.set_checked(checked);
    }
}

pub fn setup(config: &Config) -> (TrayHandle, MenuId, MenuId, MenuId, MenuId) {
    let menu = Menu::new();

    let toggle = CheckMenuItem::new("Enabled", true, config.enabled, None);
    menu.append(&toggle).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let mode_label = match config.mode {
        Mode::Api => "Switch to Mouse Jiggle",
        Mode::Mouse => "Switch to API Inhibit",
    };
    let mode_item = MenuItem::new(mode_label, true, None);
    menu.append(&mode_item).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let autostart = CheckMenuItem::new("Launch at Login", true, config.autostart, None);
    menu.append(&autostart).ok();

    menu.append(&PredefinedMenuItem::separator()).ok();

    let quit = MenuItem::new("Quit", true, None);
    menu.append(&quit).ok();

    let icon = make_icon();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip(if config.enabled {
            "Keep Awake — Enabled"
        } else {
            "Keep Awake — Disabled"
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

fn make_icon() -> Icon {
    let size = 32;
    let cx = 16;
    let cy = 16;
    let r = 12;
    let mut rgba = vec![0u8; size as usize * size as usize * 4];

    for y in 0..size {
        for x in 0..size {
            let dx = (x - cx) as f64;
            let dy = (y - cy) as f64;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = ((y * size + x) * 4) as usize;

            if dist <= r as f64 {
                let edge = dist > r as f64 - 1.5;
                let alpha = if edge {
                    ((r as f64 - dist).max(0.0) * 255.0) as u8
                } else {
                    255
                };
                if dist < 5.0 {
                    rgba[idx + 3] = alpha;
                } else if dist < r as f64 - 2.0 {
                    rgba[idx] = 76;
                    rgba[idx + 1] = 175;
                    rgba[idx + 2] = 80;
                    rgba[idx + 3] = alpha;
                } else {
                    rgba[idx + 3] = 0;
                }
            } else {
                rgba[idx + 3] = 0;
            }
        }
    }

    Icon::from_rgba(rgba, size, size).expect("Failed to create icon")
}
