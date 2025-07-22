use std::{error::Error, os::raw::{c_int}, sync::LazyLock, thread::spawn};

use inotify_sys::{inotify_add_watch, inotify_init1, IN_CLOSE_WRITE, IN_NONBLOCK};

static INOTIFY_FD: LazyLock<c_int> = LazyLock::new(|| {
    let fd = unsafe {inotify_init1(IN_NONBLOCK)};
    
    spawn(|| {
        
    });

    fd
});

pub fn register_file_watcher<F>(path: &str, f: F) -> Result<(), Box<dyn Error + Send + Sync>>
where
    F: Fn() -> ()
{
    let fd = &INOTIFY_FD;
    if **fd == -1 {
        return Err("Failed to initialize inotify, file watcher disabled".into());
    }

    unsafe {
        let wd = inotify_add_watch(**fd, path.as_bytes().as_ptr() as *const i8, IN_CLOSE_WRITE);

        if wd == -1 {
            return Err("Failed to add watcher".into());
        }
    }

    Ok(())
}
