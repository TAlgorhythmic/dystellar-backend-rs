use std::{error::Error, fs::{read, File}, io::Read, os::{fd::{FromRawFd, IntoRawFd, OwnedFd, RawFd}, raw::c_int, unix::thread::JoinHandleExt}, sync::{LazyLock, Mutex}, thread::spawn};

use inotify_sys::{inotify_add_watch, inotify_event, inotify_init, IN_CLOSE_WRITE, IN_CREATE, IN_MOVED_TO};

static WATCHERS: Mutex<Vec<Box<dyn Fn() + Send + Sync + 'static>>> = Mutex::new(vec![]);
static INOTIFY_FD: LazyLock<c_int> = LazyLock::new(|| {
    let fd = unsafe {inotify_init()};
    if fd == -1 {
        return fd;
    }

    let owned_fd = unsafe {OwnedFd::from_raw_fd(fd)};
    
    spawn(move || {
        let mut file = File::from(owned_fd);
        let mut buff = [0u8; 4096];

        loop {
            if let Ok(read) = file.read(&mut buff) {
                let mut offset = 0;

                while offset < read {
                    let event = unsafe {buff.as_ptr().add(offset)} as *const inotify_event;
                    let wd: i32 = unsafe {(*event).wd};
                    let event_size = std::mem::size_of::<inotify_event>() + unsafe {(*event).len as usize};
                    
                    offset += event_size;
                    let watchers = WATCHERS.lock().unwrap();
                    watchers[wd as usize]();
                }
            } else {
                println!("[inotify] Failed to read from inotify");
            }

        }
    });

    fd
});

pub fn register_file_watcher<F>(path: &str, f: F) -> Result<(), Box<dyn Error + Send + Sync>>
where
    F: Fn() + Send + Sync + 'static
{
    let fd = &INOTIFY_FD;
    if **fd == -1 {
        return Err("[inotify] Failed to initialize inotify, file watcher disabled".into());
    }

    unsafe {
        let wd = inotify_add_watch(**fd, path.as_bytes().as_ptr() as *const i8, IN_CLOSE_WRITE | IN_CREATE | IN_MOVED_TO);
        
        if wd < 1 {
            return Err("[inotify] Failed to add watcher".into());
        }

        let mut watchers = WATCHERS.lock().unwrap();
        if watchers.len() < (wd + 1) as usize {
            watchers.resize_with((wd + 10) as usize, || Box::new(|| {}));
        }

        watchers[wd as usize] = Box::new(f);
    }

    Ok(())
}
