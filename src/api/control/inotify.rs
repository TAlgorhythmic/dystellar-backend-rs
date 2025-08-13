use std::{error::Error, ffi::{CStr, CString}, fs::{self, File}, io::Read, os::{fd::{FromRawFd, OwnedFd}, raw::{c_char, c_int}}, sync::{Arc}};

use inotify_sys::{inotify_add_watch, inotify_event, inotify_init, IN_CLOSE_WRITE, IN_DELETE};

pub struct WatchedFile {
    name: Box<str>,
    path_ref: Arc<str>
}

pub struct DirWatcher<F> where F: Fn(&str) {
    recursive: bool,
    watch_all: bool,
    fd: i32,
    wd: c_int,
    path_base: Arc<str>,
    subwatchers: Vec<DirWatcher<F>>,
    watched_files: Option<Arc<Vec<(c_int, Arc<str>, Box<str>)>>>,
    on_modify: Arc<F>,
    on_delete: Arc<F>
}

impl<F> DirWatcher<F> where F: Fn(&str) {
    pub fn watch(&self, file: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.watch_all {
            return Ok(());
        }

        

        Ok(())
    }

    fn create_sub(fd: i32, mut directory: &str, recursive: bool, watch_all: bool, on_modify: Arc<F>, on_delete: Arc<F>, watched_files: Option<Arc<Vec<(Arc<str>, Box<str>)>>>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        if directory.ends_with('/') {
            directory = unsafe {directory.slice_unchecked(0, directory.len() - 1)};
        }

        let mut subwatchers: Vec<DirWatcher<F>> = vec![];

        if recursive {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;

                if entry.file_type()?.is_dir() {
                    subwatchers.push(Self::create_sub(fd, format!("{directory}/{}", entry.file_name().to_str().unwrap()).as_str(), recursive, watch_all, on_modify.clone(), on_delete.clone(), watched_files.clone())?);
                }
            }
        }

        let wd = unsafe {inotify_add_watch(fd, CString::new(directory)?.as_ptr(), IN_CLOSE_WRITE | IN_DELETE)};

        Ok(DirWatcher {
            recursive, watch_all, fd, wd,
            path_base: directory.into(),
            subwatchers, watched_files,
            on_modify, on_delete
        })
    }

    pub fn create(mut directory: &str, recursive: bool, watch_all: bool, on_modify: F, on_delete: F) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let fd = unsafe { inotify_init() };
        if fd == -1 {
            return Err("Failed to initialize inotify".into());
        }

        if directory.ends_with('/') {
            directory = unsafe {directory.slice_unchecked(0, directory.len() - 1)};
        }


        let watched_files = if watch_all {None} else {Some(Arc::new(vec![]))};
        let mut subwatchers: Vec<DirWatcher<F>> = vec![];

        let on_modify = Arc::new(on_modify);
        let on_delete = Arc::new(on_delete);

        if recursive {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;

                if entry.file_type()?.is_dir() {
                    subwatchers.push(Self::create_sub(fd, format!("{directory}/{}", entry.file_name().to_str().unwrap()).as_str(), recursive, watch_all, on_modify.clone(), on_delete.clone(), watched_files.clone())?);
                }
            }
        }

        let wd = unsafe {inotify_add_watch(fd, CString::new(directory)?.as_ptr(), IN_CLOSE_WRITE | IN_DELETE)};

        Ok(DirWatcher {
            recursive, watch_all, fd, wd,
            path_base: directory.into(),
            subwatchers, watched_files,
            on_modify, on_delete
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
