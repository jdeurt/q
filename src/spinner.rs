use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub fn with_spinner<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();

    let handle = thread::spawn(move || {
        let mut i = 0;
        let mut stderr = io::stderr();
        while !done_clone.load(Ordering::Relaxed) {
            let _ = write!(stderr, "\r{} ", FRAMES[i % FRAMES.len()]);
            let _ = stderr.flush();
            i += 1;
            thread::sleep(Duration::from_millis(80));
        }
        let _ = write!(stderr, "\r  \r");
        let _ = stderr.flush();
    });

    let result = f();
    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
    result
}
