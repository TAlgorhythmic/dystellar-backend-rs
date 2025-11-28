use std::{error::Error, ffi::{CStr, CString}, fs::File, io::Read, os::{fd::{FromRawFd, OwnedFd}, raw::{c_char, c_int}}};

use inotify_sys::{IN_CLOSE_WRITE, IN_DELETE, IN_MOVED_FROM, IN_MOVED_TO, inotify_add_watch, inotify_event, inotify_init};

type WatchFunction = Box<dyn Fn(&str) + Send + Sync + 'static>;

pub struct WatchedFile {
    filename: Box<str>,
    on_modify: WatchFunction,
    on_delete: Option<WatchFunction>
}

pub struct DirWatcher {
    fd: i32,
    wd: c_int,
    path_base: Box<str>,
    watched_files: Vec<WatchedFile>,
    on_modify_global: Option<WatchFunction>
}

impl DirWatcher {
    pub fn watch(&mut self, file: &str, on_modify: WatchFunction, on_delete: Option<WatchFunction>) {
        self.watched_files.push(WatchedFile { filename: file.into(), on_modify, on_delete });
    }

    pub fn watch_global(&mut self, on_modify: WatchFunction) {
        self.on_modify_global = Some(on_modify);
    }

    pub fn create(mut directory: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let fd = unsafe { inotify_init() };
        if fd == -1 {
            return Err("Failed to initialize inotify".into());
        }

        if directory.ends_with('/') {
            directory = unsafe {directory.get_unchecked(0..directory.len() - 1)};
        }

        let wd = unsafe {inotify_add_watch(fd, CString::new(directory)?.as_ptr(), IN_CLOSE_WRITE | IN_MOVED_TO | IN_MOVED_FROM | IN_DELETE)};

        Ok(DirWatcher {
            fd, wd, path_base: directory.into(),
            watched_files: vec![],
            on_modify_global: None
        })
    }

    /**
    * Consumes the instance and listens for events
    */
    pub fn listen(self) {
        std::thread::spawn(move || {
            let owned_fd = unsafe { OwnedFd::from_raw_fd(self.fd) };
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

                        let string = unsafe { CStr::from_ptr(str).to_str().unwrap() };

                        if let Some(modify_global) = &self.on_modify_global {
                            modify_global(string);
                        }
                        if let Some(watcher) = self.watched_files.iter().find(|w| w.filename.as_ref() == string) {
                            (watcher.on_modify)(string);
                        }
                    }
                } else {
                    println!("[inotify] Failed to read from inotify");
                }
            }
        });
    }
}
