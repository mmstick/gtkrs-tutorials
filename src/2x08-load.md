# Loading tasks from a file

## app.rs

If in the future, we implement the ability to open a different list, we'll need a way of clearing the UI of the previous list. This simply involves popping out every task in the map and removing them one by one.

```rust
pub fn clear(&mut self) {
    while let Some(entity) = self.tasks.keys().next() {
        self.remove_(entity);
    }
}
```

When we receive the contents of a list to load into our UI, we're going to split the string by newlines and create a row for each one, then insert that text into their entries.

```rust
pub fn load(&mut self, data: String) {
    self.clear();

    for (row, line) in data.lines().enumerate() {
        let entity = self.insert_row(row as i32);
        self.tasks[entity].set_text(line);
    }
}
```

## widgets.rs

Because we are automatically filling out the contents of the `Task::entry` for each task that we load from a file, and we are listening to any changes made to these entries when we send the `Modified` event, we need to block that signal when we are setting the text in the entry. Add the following new property to `Task`:

```rust
entry_signal: Option<SignalHandlerId>,
```

Which we'll need to assign to `None` in our `Task::new()` method. Then change the `connect_changed` signal for the entry to the following:

```rust
let signal = self.entry.connect_changed(clone!(@strong tx => move |_| {
    let tx = tx.clone();
    spawn(async move {
        let _ = tx.send(Event::Modified).await;
    });
}));

self.entry_signal = Some(signal);
```

Now we can safely add a method for setting the text on this entry, first by blocking that signal, setting the text, and unblocking it:

```rust
pub fn set_text(&mut self, text: &str) {
    let signal = self.entry_signal.as_ref().unwrap();
    self.entry.block_signal(signal);
    self.entry.set_text(text);
    self.entry.unblock_signal(signal);
}
```