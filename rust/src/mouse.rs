use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct MouseJiggler {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl MouseJiggler {
    pub fn start(interval_secs: u64, running: Arc<AtomicBool>) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();

        let handle = thread::spawn(move || {
            while !stop_clone.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(interval_secs));

                if !running.load(Ordering::Relaxed) || stop_clone.load(Ordering::Relaxed) {
                    break;
                }

                if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                    if let Ok(pos) = enigo.location() {
                        let _ = enigo.move_mouse(pos.0 + 1, pos.1, Coordinate::Abs);
                        thread::sleep(Duration::from_millis(20));
                        let _ = enigo.move_mouse(pos.0, pos.1, Coordinate::Abs);
                    }
                }
            }
        });

        Self { stop, handle: Some(handle) }
    }
}

impl Drop for MouseJiggler {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
