# Marking & Removing Done Tasks with CheckButtons

We are going to remove the remove buttons from each task and replace them with check buttons. To remove tasks from the list, we will replace the application's title bar with a `gtk::HeaderBar`, and place a delete button here that will show when any tasks have been checked.

## main.rs

We're adding two new events to our `Event`:

```rust
Delete,
Toggled(bool),
```

Handling it in our event handler:

```rust
Event::Toggled(active) => app.toggled(active),
Event::Delete => app.delete(),
```

## widgets.rs

We can track if tasks are completed with check marks, and remove them together in a batch. Add a new field to our `Task` struct to add a `gtk::CheckButton`:

```rust
check: gtk::CheckButton
```

Since we're going to use these check marks for removal operations, we can remove the `remove` button as well.

Then construct the widget and return it:

```rust
Self {
    check: cascade! {
        gtk::CheckButton::new();
        ..show();
    },

    insert: cascade! {
        gtk::Button::from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button);
        ..show();
    },

    entry: cascade! {
        gtk::Entry::new();
        ..set_hexpand(true);
        ..show();
    },

    entry_signal: None,
    row,
}
```

Then we can add an event for when the button is toggled:


```rust
self.check.connect_toggled(clone!(@strong tx => move |check| {
    let tx = tx.clone();
    let check = check.clone();
    spawn(async move {
        let _ = tx.send(Event::Toggled(check.get_active())).await;
    })
}));
```

## app.rs

### Task Widgets

Then modify the attachments of these widgets in the app:

```rust
self.container.attach(&task.check, 0, row, 1, 1);
self.container.attach(&task.entry, 1, row, 1, 1);
self.container.attach(&task.insert, 2, row, 1, 1);
```

### Delete Button

Now we're going to create the delete button, with both an icon and a label. By default, a button is only permitted to have either an image or a label, but we can force it to show both by setting the `always_show_image` property. We also don't want this button to be shown when the window is shown, so we need to call `.set_no_show_all(true)`. Since this button performs a destructive action, we should style it as such with `.get_style_context().add_class(&gtk::STYLE_CLASS_DESTRUCTIVE_ACTION)`.

```rust
let delete_button = cascade! {
    gtk::Button::from_icon_name(Some("edit-delete-symbolic"), gtk::IconSize::Button);
    ..set_label("Delete");
    // Show the icon alongside the label
    ..set_always_show_image(true);
    // Don't show this when the window calls `.show_all()`
    ..set_no_show_all(true);
    // Give this a destructive styling to signal that the action is destructive
    ..get_style_context().add_class(&gtk::STYLE_CLASS_DESTRUCTIVE_ACTION);
    // Send the `Delete` event on click
    ..connect_clicked(clone!(@strong tx => move |_| {
        let tx = tx.clone();
        spawn(async move {
            let _ = tx.send(Event::Delete).await;
        });
    }));
};
```

This button widget will be attached to the title bar via the `gtk::HeaderBar`:

```rust
let headerbar = cascade! {
    gtk::HeaderBar::new();
    ..pack_end(&delete_button);
    ..set_title(Some("ToDo"));
    ..set_show_close_button(true);
};
```

Then modify our `ApplicationWindow` to change `.set_title()` for the following:

```rust
..set_titlebar(Some(&headerbar));
```

And update our `App` struct to add the delete button.

```rust
delete_button: gtk::Button
```

### Handling Toggle Events

We will show the delete button only when there is at least one active task checked. We can achieve this by adding another property to the `App` struct to track how many tasks are actively checked.

```rust
checks_active: u32
```

By default, assigning it in our constructor to `0` of course

```rust
checks_active: 0
```

Then we'll add the toggled method for handling the toggle events. If the event is toggled active, we increment the number. We do the reverse when it is unchecked. If the number is non-zero, we set the button as visible.

```rust
pub fn toggled(&mut self, active: bool) {
    if active {
        self.checks_active += 1;
    } else {
        self.checks_active -= 1;
    }

    self.delete_button.set_visible(self.checks_active != 0);
}
```

### Handling Delete Events

When we've been requested to delete tasks that were marked as active, we'll iterate through our tasks and collect the entity IDs of each task that is active. We need to collect these into a vector on the side so that we're not modifying our task list as we're iterating across it. Once we have a list of tasks to remove, we'll call our remove method with each entity ID. Finally, we'll set the `checks_active` back to `0` and hide the button.

```rust
pub fn delete(&mut self) {
    let remove_list = self.tasks.iter()
        .filter(|(_, task)| task.check.get_active())
        .map(|(id, _)| id)
        .collect::<Vec<TaskEntity>>();

    for id in remove_list {
        self.remove(id);
    }

    self.checks_active = 0;
    self.delete_button.set_visible(false);
}
```