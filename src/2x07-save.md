# Signaling when to Save

## app.rs

There are two scenarios where we will save our tasks to a file. When the application has been closed, and every 5 seconds after the last modification. To start, lets add a signal that waits 5 seconds before sending the `SyncToDisk` event:

```rust
pub fn modified(&mut self) {
    if let Some(id) = self.scheduled_write.take() {
        glib::source_remove(id);
    }

    let tx = self.tx.clone();
    self.scheduled_write = Some(glib::timeout_add_local(5000, move || {
        let tx = tx.clone();
        spawn(async move {
            let _ = tx.send(Event::SyncToDisk).await;
        });

        glib::Continue(false)
    }));
}
```

`glib::timeout_add_local(5000, ...)` will schedule the provided closure to execute on local context after 5 seconds. This function returns an ID which we're storing in the `scheduled_write` property of our `App`. If we receive the `Modified` event again before the 5 seconds have passed, the previous signal will be removed and a new one registered in its place. That'll ensure that it doesn't trigger until after 5 seconds of idle keyboard time has passed.

Next will be programming the `SyncToDisk` event. We're simply going to collect the text from each non-empty task widget in our slotmap, and combine it into a single string to pass to the background for saving. The `fomat` formatter from the `fomat_macros` crate provides a very intuitive means to achieve this.

```rust
pub async fn sync_to_disk(&mut self) {
    self.scheduled_write = None;

    let contents = fomat_macros::fomat!(
        for node in self.tasks.values() {
            if node.entry.get_text_length() != 0 {
                (node.entry.get_text()) "\n"
            }
        }
    );

    let _ = self.btx.send(BgEvent::Save("Task".into(), contents)).await;
}
```

Finally, we can handle that `Closed` event that was sent when the `ApplicationWindow` was destroyed:

```rust
pub async fn closed(&mut self) {
    self.sync_to_disk().await;
    let _ = self.btx.send(BgEvent::Quit).await;
}
```