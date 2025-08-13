use std::{error::Error, ffi::{CStr, CString}, fs::{self, File}, io::Read, os::{fd::{FromRawFd, OwnedFd}, raw::{c_char, c_int}}, sync::{Arc}};

use inotify_sys::{inotify_add_watch, inotify_event, inotify_init, IN_CLOSE_WRITE, IN_DELETE};

pub struct WatchedFile<F> where F: Fn(&str) {
    filename: Box<str>,
    on_modify: F,
    on_delete: F
}

pub struct DirWatcher<F> where F: Fn(&str) {
    fd: i32,
    wd: c_int,
    path_base: Box<str>,
    watched_files: Vec<WatchedFile<F>>,
}

impl<F> DirWatcher<F> where F: Fn(&str) {
    pub fn watch(&self, file: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    }

    pub fn create(mut directory: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let fd = unsafe { inotify_init() };
        if fd == -1 {
            return Err("Failed to initialize inotify".into());
        }

        if directory.ends_with('/') {
            directory = unsafe {directory.slice_unchecked(0, directory.len() - 1)};
        }

        let wd = unsafe {inotify_add_watch(fd, CString::new(directory)?.as_ptr(), IN_CLOSE_WRITE | IN_DELETE)};

        Ok(DirWatcher {
            fd, wd, path_base: directory.into(),
            watched_files: vec![]
        })
    }
}

fn listen_events(fd: i32) {
    std::thread::spawn(move || {
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
