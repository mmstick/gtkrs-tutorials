# Creating the Task Widget Struct

Before we work on the core of the application itself, I typically start by creating the widgets which the application will build upon. Every task in the application will consist of three widgets:

- `gtk::Entry` for editing the text of a task
- `gtk::Button` for inserting a new task below this task
- `gtk::Button` for removing this task

## widgets.rs

Take note that because our tasks are created dynamically at runtime, we'll want a way of tracking them, which we can achieve with a `SlotMap`. The `Task` struct is going to contain each of the widgets owned by this task, as well as the row where this task was stored.

```rust
use crate::{utils::spawn, Event, TaskEntity};
use async_channel::Sender;
use glib::{clone, SignalHandlerId};
use gtk::prelude::*;

pub struct Task {
    pub entry: gtk::Entry,
    pub insert: gtk::Button,
    pub remove: gtk::Button,

    // Tracks our position in the list
    pub row: i32,
}
```

Now we can construct our widgets.


```rust
impl Task {
    pub fn new(row: i32) -> Self {

    }
}
```

The text entry will horizontally expanded to consume as much space as possible

```rust
let entry = cascade! {
    gtk::Entry::new();
    ..set_hexpand(true);
    ..show();
};
```

Then we'll create our two buttons. It's good practice to use icons over text, because text requires translations.

```rust
let insert = cascade! {
    gtk::Button::from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button);
    ..show();
};

let remove = cascade! {
    gtk::Button::from_icon_name(Some("list-remove-symbolic"), gtk::IconSize::Button);
    ..show();
};
```

We're not going to program these widgets just yet. Just return them as is to program later:

```rust
impl Task {
    pub fn new(row: i32) -> Self {
        Self {
            insert: cascade! {
                gtk::Button::from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button);
                ..show();
            },

            remove: cascade! {
                gtk::Button::from_icon_name(Some("list-remove-symbolic"), gtk::IconSize::Button);
                ..show();
            },

            entry: cascade! {
                gtk::Entry::new();
                ..set_hexpand(true);
                ..show();
            },

            row,
        }
    }
}
```

You can find available system icons for your applications using [IconLibray](https://www.flathub.org/apps/details/org.gnome.design.IconLibrary).