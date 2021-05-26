# Loading and Saving in the Background

## main.rs

To handle events in the background, and within the app itself, we will need two separate channels. One receiver will listen for application events in the main thread which manages the UI. The other receiver will listen for events from the application in a background thread.

```rust
// Channel for UI events in the main thread
let (tx, rx) = async_channel::unbounded();

// Channel for background events to the background thread
let (btx, brx) = async_channel::unbounded();
```

Reading and writing data to a file is a blocking operation that has risk of freezing the application when these operations are occurring on the same thread as the UI. We can therefore avoid hanging the UI simply by passing these tasks off to a background thread.

## Spawning the background thread

Next we will spawn a thread, and pass both a clone of our application event sender, and the background event receiver. The glib crate provides a [clone macro](https://gtk-rs.org/docs/glib/macro.clone.html#passing-a-strong-reference) which can be used

```rust
// Take ownership of a copy of the UI event sender (tx),
// and the background event receiver (brx).
std::thread::spawn(glib::clone!(@strong tx => move || {
    // Fetch the executor registered for this thread
    utils::thread_context()
        // Block this thread on an event loop future
        .block_on(background::run(tx, brx));
}));
```

We're going to attach the background sender to our `App` in the future, so we need to update our call to `App::new()` to take both channels as input parameters.

```rust
let mut app = App::new(app, tx, btx);
```

## background.rs

Our background event loop is going to start with an async function that looks like this. It all take a sender for events we need to pass back to the UI, and the receiver for receiving events from the UI.


```rust
use crate::Event;
use async_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

pub async fn run(tx: Sender<Event>, rx: Receiver<BgEvent>) {

}
```

### XDG

On first startup, our application will load the most recently-modified task in memory. Applications should adhere to the XDG standards when they are making decisions about where to store files used by their application. Using the `xdg` crate, we can get the prefix for your application with the following code:

```rust
let xdg_dirs = xdg::BaseDirectories::with_prefix(crate::APP_ID)
    .unwrap();
```

Because the directory will not exist on a first startup, we need to ensure it's created:


```rust
let data_home = xdg_dirs.get_data_home();

let _ = fs::create_dir_all(&data_home);
```

With the data directory for our app now created, we'll search it for the most recently-created file in this directory, and read that file into memory to pass back to our app:

```rust
if let Some(path) = most_recent_file(&data_home).unwrap() {
    if let Ok(data) = std::fs::read_to_string(&path) {
        let _ = tx.send(Event::Load(data)).await;
    }
}
```

### Fetching the most-recent file

The function, `most_recent_file()` contains the following for reference:

```rust
fn most_recent_file(path: &Path) -> io::Result<Option<PathBuf>> {
    let mut most_recent = SystemTime::UNIX_EPOCH;
    let mut target = None;

    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        if entry.file_type().map_or(false, |kind| kind.is_file()) {
            if let Ok(modified) = entry.metadata()
                .and_then(|m| m.modified())
            {
                if modified > most_recent {
                    target = Some(entry.path());
                    most_recent = modified;
                }
            }
        }
    }

    Ok(target)
}
```

### Handling Events

And then finally, we will start handling the events we receive from the UI. The first being a request to save notes to a file, and the other a request to quit the application.

```rust
/// Events that the background thread's event loop will respond to
pub enum BgEvent {
    // Save tasks to a file
    Save(PathBuf, String),

    // Exit the from the event loop
    Quit
}
```

The `Quit` event will break from the event loop and then reply to the application that we have finished any task we were waiting on, and it is now safe to exit the application.

```rust
while let Ok(event) = rx.recv().await {
    match event {
        BgEvent::Save(path, data) => {
            let path = xdg_dirs.place_data_file(path).unwrap();
            std::fs::write(&path, data.as_bytes()).unwrap();
        },

        BgEvent::Quit => break
    }
}

let _ = tx.send(Event::Quit).await;
```

### Review

At the end, your file should look like this:

```rust
pub async fn run(tx: Sender<Event>, rx: Receiver<BgEvent>) {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(crate::APP_ID).unwrap();

    let data_home = xdg_dirs.get_data_home();

    let _ = fs::create_dir_all(&data_home);

    if let Some(path) = most_recent_file(&data_home).unwrap() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            let _ = tx.send(Event::Load(data)).await;
        }
    }

    while let Ok(event) = rx.recv().await {
        match event {
            BgEvent::Save(path, data) => {
                let path = xdg_dirs.place_data_file(path).unwrap();
                std::fs::write(&path, data.as_bytes()).unwrap();
            },

            BgEvent::Quit => break
        }
    }

    let _ = tx.send(Event::Quit).await;
}
```