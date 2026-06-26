use std::sync::OnceLock;

static CURRENT_LANG: OnceLock<Lang> = OnceLock::new();

#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    Zh,
    En,
}

pub fn detect() -> Lang {
    *CURRENT_LANG.get_or_init(|| {
        // Try LANG env var (macOS / Linux)
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("zh") {
                return Lang::Zh;
            }
        }
        // Windows: try PowerShell to get culture
        #[cfg(windows)]
        {
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", "(Get-Culture).Name"])
                .output()
                .ok();
            if let Some(out) = out {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if s.starts_with("zh") {
                    return Lang::Zh;
                }
            }
        }
        Lang::En
    })
}

#[macro_export]
macro_rules! tr {
    ($en:literal) => {{
        let lang = $crate::lang::detect();
        match lang {
            $crate::lang::Lang::En => $en,
            $crate::lang::Lang::Zh => $en, // fallback, overridden below
        }
    }};
    ($en:literal, $zh:literal) => {{
        let lang = $crate::lang::detect();
        match lang {
            $crate::lang::Lang::En => $en,
            $crate::lang::Lang::Zh => $zh,
        }
    }};
}
