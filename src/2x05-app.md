# Creating the App

## app.rs

Now we can get to creating our `App` struct. This will contain all of the values that we will work with throughout the lifetime of our application.

```rust
use crate::{Event, BgEvent, TaskEntity};
use crate::widgets::Task;
use crate::utils::spawn;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;

pub struct App {
    pub container: gtk::Grid,
    pub tasks: SlotMap<TaskEntity, Task>,
    pub scheduled_write: Option<SourceId>,
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
}
```

All of our task widgets are going to be stored within the `container: gtk::Grid`. Each row of this grid will be associated with a task. The first column will contain the `gtk::Entry`, and the subsequent two columsn are the `gtk::Button`s. By using a grid, we can easily keep our widgets perfectly aligned in a grid.

The `tasks: SlotMap<TaskEntity, Task>` field will contain all the tasks we're currently maintaining. This will be important for looking up which row a task was assigned to.

When an entry has been modified, we're going to spawn a signal that waits until 5 seconds have passed since the last modification before sending an event to the background thread to save the contents of our task list, whose source ID is stored in `scheduled_write: Option<SourceId>`.

And without requiring much explanation, `tx` and `btx` are handles for sending UI and background events.

### Setting up the App

```rust
impl App {
    pub fn new(
        app: &gtk::Application,
        tx: Sender<Event>,
        btx: Sender<BgEvent>
    ) -> Self {

    }
}
```

The first step will be creating the `gtk::Grid` that we are going to assign our widgets to. Each column and row will have 4 units of padding around them, and the widget itself will also have some padding.

```rust
let container = cascade! {
    gtk::Grid::new();
    ..set_column_spacing(4);
    ..set_row_spacing(4);
    ..set_border_width(4);
    ..show();
};
```

Because it will be possible for there to be more tasks than a window can display at one time, this widget will be wrapped within a `gtk::ScrolledWindow`. By defining that the `hscrollbar-policy` is `Never`, this will prevent the scrolling window from horizontally scrolling, but will permit vertical scrolling as necessary.

```rust
let scrolled = gtk::ScrolledWindowBuilder::new()
    .hscrollbar_policy(gtk::PolicyType::Never)
    .build();

scrolled.add(&container);
```

Now we get to setting up our window, which we can create from the `&gtk::Application` we received. Note that we are connecting our sender along with the scroller to the delete event. When the window is being destroyed, we are going to detach the scroller from the window so that it does not get destroyed alongside it. The purpose of doing so is to keep our `gtk::Entry` task widgets alive long enough for us to salvage the text in them to save their contents to the disk before we exit the application. Our sender is going to pass a UI event notifying our event handler about the window having been closed.

```rust
let _window = cascade! {
    gtk::ApplicationWindow::new(app);
    ..set_title("Todo");
    ..add(&scrolled);
    ..connect_delete_event(clone!(@strong tx, @strong scrolled => move |win, _| {
        // Detach to preserve widgets after destruction of window
        win.remove(&scrolled);

        let tx = tx.clone();
        spawn(async move {
            let _ = tx.send(Event::Closed).await;
        });
        gtk::Inhibit(false)
    }));
    ..show_all();
};

gtk::Window::set_default_icon_name("icon-name-here");
```

The last step is putting our app together, creating the first row, and returning the `App` struct:

```rust
let mut app = Self {
    container,
    tasks: SlotMap::with_key(),
    scheduled_write: None,
    tx,
    btx,
};

app.insert_row(0);

app
```

Your file should now look like so:

```rust
use crate::{Event, BgEvent, TaskEntity};
use crate::widgets::Task;
use crate::utils::spawn;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;

pub struct App {
    pub container: gtk::Grid,
    pub tasks: SlotMap<TaskEntity, Task>,
    pub scheduled_write: Option<SourceId>,
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
}

impl App {
    pub fn new(app: &gtk::Application, tx: Sender<Event>, btx: Sender<BgEvent>) -> Self {
        let container = cascade! {
            gtk::Grid::new();
            ..set_column_spacing(4);
            ..set_row_spacing(4);
            ..set_border_width(4);
            ..show();
        };

        let scrolled = gtk::ScrolledWindowBuilder::new()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();

        scrolled.add(&container);

        let _window = cascade! {
            gtk::ApplicationWindow::new(app);
            ..set_title("Todo");
            ..set_default_size(400, 600);
            ..add(&scrolled);
            ..connect_delete_event(clone!(@strong tx, @strong scrolled => move |win, _| {
                // Detach to preserve widgets after destruction of window
                win.remove(&scrolled);

                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Closed).await;
                });
                gtk::Inhibit(false)
            }));
            ..show_all();
        };

        gtk::Window::set_default_icon_name("icon-name-here");

        let mut app = Self {
            container,
            tasks: SlotMap::with_key(),
            scheduled_write: None,
            tx,
            btx,
        };

        app.insert_row(0);

        app
    }
}
```