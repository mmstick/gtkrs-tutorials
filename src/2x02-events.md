# Modeling Our Events

Before going to the next step, we need to think about how we will design our application, and what events our application is going to handle.

A ToDo application will have the following behaviors:

- Insert a task
- Remove a task

Each task will be represented in our UI as a row containing the following widgets: A `gtk::Entry` for writing our task notes; with two `gtk::Button`s for inserting a new task below, or removing the task in that row. Our tasks will be stored in a `SlotMap`, where each task is referenced by their custom key: `TaskEntity`.

- Load tasks from a file

When we load our notes from a file, it will be in the form of a `String`, and each task will be a separate line in that string. The application will automatically create a new task row for each line in that string.

- Notify when a task is modified
- Save tasks to a file

Every 5 seconds after the last modification, we will fetch the contents of each `gtk::Entry` and save them to a file. We will also save the contents of each widget when the application has been closed.

- Notify that the application has been closed
- Notify that we are ready to quit

The last two events are an important distinction. When the GTK application has been closed, we will get notified that it has been destroyed. During that time, we will schedule to have our notes saved, and quit the application once they've been saved to the disk.

## main.rs

```rust
// Create a key type to identify the keys that we'll use for the Task SlotMap.
slotmap::new_key_type! {
    pub struct TaskEntity;
}

pub enum Event {
    // Insert a task below the given task, identified by its key
    Insert(TaskEntity),

    // A previous task list has been fetched from a file from the background
    // thread, and it is now our job to display it in our UI.
    Load(String),

    // Signals that an entry was modified, and at some point we should save it
    Modified,

    // Removes the task identified by this entity
    Remove(TaskEntity),

    // Signals that we should collect up the text from each task and pass it
    // to a background thread to save it to a file.
    SyncToDisk,

    // Signals that the window has been closed, so we should clean up and quit
    Closed,

    // Signals that the process has saved to disk and it is safe to exit
    Quit,
}
```

Then modify our event handler like so:

```rust
let event_handler = async move {
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Modified => app.modified(),
            Event::Insert(entity) => app.insert(entity),
            Event::Remove(entity) => app.remove(entity),
            Event::SyncToDisk => app.sync_to_disk().await,
            Event::Load(data) => app.load(data),
            Event::Closed => app.closed().await,
            Event::Quit => gtk::main_quit(),
        }
    }
};
```

Events are listed in the order that they are most-likely to be called in, with the most-called events first.