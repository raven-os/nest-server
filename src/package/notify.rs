use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use failure::Error;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use super::NPFManager;
use crate::config::Config;

pub fn async_watch_fs(config: Arc<Config>, npf_manager: Arc<RwLock<NPFManager>>) {
    let config = config.clone();
    let package_dir = config.package_dir().to_path_buf();

    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher =
            watcher(tx, Duration::from_secs(1)).expect("failed to create the filesystem watcher");

        watcher
            .watch(package_dir, RecursiveMode::Recursive)
            .expect("failed to watch the filesystem");

        loop {
            let res: Result<_, Error> = try {
                match rx.recv() {
                    Ok(DebouncedEvent::Create(path)) => {
                        println!("[NOTIFY] @Create {}", path.display());

                        let npf_manager = npf_manager.clone();
                        let mut npf_manager = npf_manager
                            .write()
                            .expect("can't open the NPF manager in read-write mode");

                        npf_manager.flush(&path)?;
                    }
                    Ok(DebouncedEvent::Remove(path)) => {
                        println!("[NOTIFY] @Remove {}", path.display());

                        let npf_manager = npf_manager.clone();
                        let mut npf_manager = npf_manager
                            .write()
                            .expect("can't open the NPF manager in read-write mode");

                        npf_manager.flush(&path)?;
                    }
                    Ok(DebouncedEvent::Write(path)) => {
                        println!("[NOTIFY] @Write to {}", path.display());

                        let npf_manager = npf_manager.clone();
                        let mut npf_manager = npf_manager
                            .write()
                            .expect("can't open the NPF manager in read-write mode");

                        npf_manager.flush(&path)?;
                    }
                    Ok(DebouncedEvent::Rename(old, new)) => {
                        println!("[NOTIFY] @Rename {} -> {}", old.display(), new.display());

                        let npf_manager = npf_manager.clone();
                        let mut npf_manager = npf_manager
                            .write()
                            .expect("can't open the NPF manager in read-write mode");

                        // Flush both but return the error of the first that failed
                        let r1 = npf_manager.flush(&old);
                        let r2 = npf_manager.flush(&new);

                        r1.or(r2)?;
                    }
                    Err(e) => eprintln!("[NOTIFY] Watch error: {:?}", e),
                    _ => (),
                }
            };

            if let Err(e) = res {
                eprintln!("[NOTIFY] Cache update failed: {:?}", e);
            }
        }
    });
}
