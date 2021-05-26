use crate::Event;
use async_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

pub enum BgEvent {
    Load(PathBuf),

    // Save tasks to a file
    Save(PathBuf, String),

    // Exit the from the event loop
    Quit
}

pub async fn run(tx: Sender<Event>, rx: Receiver<BgEvent>) {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(crate::APP_ID).unwrap();

    let data_home = xdg_dirs.get_data_home();

    let _ = fs::create_dir_all(&data_home);

    if let Some(path) = most_recent_file(&data_home).unwrap() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            let _ = tx.send(Event::Load(path, data)).await;
        }
    }

    while let Ok(event) = rx.recv().await {
        match event {
            BgEvent::Load(path) => {
                println!("loading {:?}", path);
                if let Ok(data) = std::fs::read_to_string(&path) {
                    let _ = tx.send(Event::Load(path, data)).await;
                }
            },

            BgEvent::Save(path, data) => {
                let path = xdg_dirs.place_data_file(path).unwrap();
                std::fs::write(&path, data.as_bytes()).unwrap();
            },

            BgEvent::Quit => break
        }
    }

    let _ = tx.send(Event::Quit).await;
}

fn most_recent_file(path: &Path) -> io::Result<Option<PathBuf>> {
    let mut most_recent = SystemTime::UNIX_EPOCH;
    let mut target = None;

    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        if entry.file_type().map_or(false, |kind| kind.is_file()) {
            if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                if modified > most_recent {
                    target = Some(entry.path());
                    most_recent = modified;
                }
            }
        }
    }

    Ok(target)
}
