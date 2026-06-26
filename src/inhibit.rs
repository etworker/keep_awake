#[cfg(windows)]
mod imp {
    pub struct InhibitGuard;

    impl InhibitGuard {
        pub fn acquire() -> Option<Self> {
            const ES_CONTINUOUS: u32 = 0x8000_0000;
            const ES_SYSTEM_REQUIRED: u32 = 0x0000_0001;
            const ES_DISPLAY_REQUIRED: u32 = 0x0000_0002;

            unsafe {
                let prev = SetThreadExecutionState(
                    ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED,
                );
                if prev != 0 { Some(Self) } else { None }
            }
        }
    }

    impl Drop for InhibitGuard {
        fn drop(&mut self) {
            const ES_CONTINUOUS: u32 = 0x8000_0000;
            unsafe { SetThreadExecutionState(ES_CONTINUOUS); }
        }
    }

    unsafe extern "system" {
        fn SetThreadExecutionState(flags: u32) -> u32;
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use core_foundation::string::CFString;
    use std::ffi::c_void;

    pub struct InhibitGuard {
        assertion_id: u32,
    }

    impl InhibitGuard {
        pub fn acquire() -> Option<Self> {
            let at = CFString::new("PreventUserIdleDisplaySleep");
            let name = CFString::new("keep-awake");
            let mut assertion_id: u32 = 0;

            unsafe {
                let ret = IOPMAssertionCreateWithName(
                    at.as_CFTypeRef() as *const c_void,
                    kIOPMAssertionLevelOn,
                    name.as_CFTypeRef() as *const c_void,
                    &mut assertion_id,
                );
                if ret == kIOReturnSuccess {
                    Some(Self { assertion_id })
                } else {
                    None
                }
            }
        }
    }

    impl Drop for InhibitGuard {
        fn drop(&mut self) {
            unsafe { IOPMAssertionRelease(self.assertion_id); }
        }
    }

    const kIOPMAssertionLevelOn: u32 = 255;
    const kIOReturnSuccess: i32 = 0;

    unsafe extern "C" {
        fn IOPMAssertionCreateWithName(
            assertion_type: *const c_void,
            assertion_level: u32,
            assertion_name: *const c_void,
            assertion_id: *mut u32,
        ) -> i32;

        fn IOPMAssertionRelease(assertion_id: u32) -> i32;
    }
}

#[cfg(target_os = "linux")]
mod imp {
    use std::time::Duration;

    pub struct InhibitGuard {
        cookie: u32,
    }

    impl InhibitGuard {
        pub fn acquire() -> Option<Self> {
            let conn = zbus::blocking::Connection::system().ok()?;
            let proxy = conn.with_proxy(
                "org.freedesktop.ScreenSaver",
                "/ScreenSaver",
                "org.freedesktop.ScreenSaver",
                Duration::from_secs(5),
            );
            let cookie: u32 = proxy
                .call("Inhibit", ("keep-awake", "Prevent idle sleep"))
                .ok()?;
            Some(Self { cookie })
        }
    }

    impl Drop for InhibitGuard {
        fn drop(&mut self) {
            if let Ok(conn) = zbus::blocking::Connection::system() {
                let proxy = conn.with_proxy(
                    "org.freedesktop.ScreenSaver",
                    "/ScreenSaver",
                    "org.freedesktop.ScreenSaver",
                    Duration::from_secs(5),
                );
                let _: Result<(), _> = proxy.call("UnInhibit", (self.cookie,));
            }
        }
    }
}

pub use imp::InhibitGuard;
