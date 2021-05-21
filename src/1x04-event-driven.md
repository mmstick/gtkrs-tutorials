# Event-Driven Approach

In the event-driven approach, event handlers will capture and control access to application state. Widgets will have their signals connected to send events through a channel to these event handlers, without access to the global application state themselves. States can be moved and exclusively owned by their respective event handlers; thereby eliminating the need for reference counters, or the need to share your application states with every widget's signal.

## Setting it up

To achieve this, we need an async channel that we can get from the `async-channel` crate:

```rust
let (tx, rx) = async_channel::unbounded();
```

Now we need some event variants that our channel will emit:

```rust
enum Event {
    Clicked
}
```

Then we will attach the receiver to a future which merely loops on our receiver forever:

```rust
let event_handler = async move {
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Clicked => {

            }
        }
    }
};
```

And spawn this event handler on the default executor:

```rust
// GLib has an executor in the background that will
// asynchronously handle our events on this thread
glib::MainContext::default().spawn_local(event_handler);
```

Your source code should now look like so, and you are now ready to continue to setting up a window with a clickable button.

```rust
#[macro_use]
extern crate cascade;

use gtk::prelude::*;
use std::process;

enum Event {
    Clicked
}

fn main() {
    glib::set_program_name("First GTK App".into());
    glib::set_application_name("First GTK App");

    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Attach `tx` to our widgets, and `rx` to our event handler
    let (tx, rx) = async_channel::unbounded();

    // Processes all application events received from signals
    let event_handler = async move {
        while let Ok(event) = rx.recv().await {
            match event {
                Event::Clicked => {

                }
            }
        }
    };

    // GLib has an executor in the background that will
    // asynchronously handle our events on this thread
    glib::MainContext::default().spawn_local(event_handler);

    // Thread will block here until the application is quit
    gtk::main();
}
```

## Avoid blocking the default executor

Take careful note that because this async task has been spawned on the same executor as all of GTK's own tasks, you must be careful to avoid doing anything that would block your event handler. If your event handler were to block, the default `MainContext` would block with it, thereby freezing the GUI.

Tasks that require a lot of CPU and/or I/O should be performed in a background thread. Generally, the only code that should be executed on the default context is code that interacts directly with the widgets — fetching information from widgets, creating new widgets, and updating existing ones.