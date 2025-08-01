use std::{collections::HashMap, error::Error, ffi::{CStr, CString}, fs::File, io::Read, os::{fd::{FromRawFd, OwnedFd}, raw::{c_char, c_int}}, sync::{LazyLock, Mutex}};

use inotify_sys::{close, inotify_add_watch, inotify_event, inotify_init, IN_CLOSE_WRITE};

static WATCHERS: LazyLock<Mutex<HashMap<Box<str>, Box<dyn Fn() + Send + Sync + 'static>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static INOTIFY_FD: LazyLock<c_int> = LazyLock::new(|| {
    let fd = unsafe {inotify_init()};
    if fd == -1 {
        return fd;
    }


    let res = unsafe { inotify_add_watch(fd, CString::new(".").unwrap().as_ptr(), IN_CLOSE_WRITE) };
    if res < 0 {
        eprintln!("[inotify] Failed to add watcher");
        unsafe { close(fd) };
        return -1;
    }

    listen_events(fd);
    fd
});

fn listen_events(fd: i32) {
    tokio::task::spawn_blocking(move || {
        let owned_fd = unsafe { OwnedFd::from_raw_fd(fd) };
        let mut file = File::from(owned_fd);
        let mut buff = [0u8; 4096];

        loop {
            if let Ok(read) = file.read(&mut buff) {
                let mut offset = 0;

                while offset < read {
                    let event = unsafe {buff.as_ptr().add(offset)} as *const inotify_event;
                    let event_size = std::mem::size_of::<inotify_event>() + unsafe {(*event).len as usize};

                    let str = unsafe { buff.as_ptr().add(std::mem::size_of::<inotify_event>() + offset) as *const c_char };
                    offset += event_size;

                    let watchers = WATCHERS.lock().unwrap();
                    let string = unsafe { CStr::from_ptr(str) };

                    if let Some(watcher) = watchers.get(string.to_str().unwrap().into()) {
                        watcher();
                    }
                }
            } else {
                println!("[inotify] Failed to read from inotify");
            }
        }
    });
}

pub fn register_file_watcher<F>(path: &str, f: F) -> Result<(), Box<dyn Error + Send + Sync>>
where
    F: Fn() + Send + Sync + 'static
{
    if *INOTIFY_FD == -1 {
        return Err("[inotify] Failed to initialize inotify, file watcher disabled".into());
    }

    let mut watchers = WATCHERS.lock().unwrap();
    watchers.insert(path.into(), Box::new(f));

    Ok(())
}
