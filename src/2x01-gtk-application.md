# Using gtk::Application

## utils.rs

Before we begin, we need to add some utility functions that we'll be using throughout the application. We will be spawning a background thread and executing tasks on it, so we'll need a convenience function for fetching the thread-default context. Likewise, we're going to spawning tasks on the global default context, so we'll need that here as well.

```rust
use std::future::Future;

pub fn thread_context() -> glib::MainContext {
    glib::MainContext::thread_default()
        .unwrap_or_else(|| {
            let ctx = glib::MainContext::new();
            ctx.push_thread_default();
            ctx
        })
}

pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static {
    glib::MainContext::default().spawn_local(future);
}
```

## main.rs

This time we will create a GTK application using the proper `gtk::Application` setup process. This will take care of initializing GTK for you, and registers your application with an application ID so that you can prevent your application from spawning multiple instances.

```rust
#[macro_use]
extern crate cascade;

mod app;
mod background;
mod widgets;
mod utils;

use self::app::App;
use gio::prelude::*;

/// The name that we will register to the system to identify our application
pub const APP_ID: &str = "io.github.mmstick.ToDo";

fn main() {
    let app_name = "Todo";

    glib::set_program_name(Some(app_name));
    glib::set_application_name(app_name);

    // Initializes GTK and registers our application. gtk::Application helps us
    // set up an application with less work
    let app = gtk::Application::new(
        Some(APP_ID),
        Default::default()
    ).expect("failed to init application");

    // After the application has been registered, it will trigger an activate
    // signal, which will give us the okay to construct our application and set
    // up our application logic. We're going to use `app` to create the
    // application window in the future.
    app.connect_activate(|app| {
        let (tx, rx) = async_channel::unbounded();

        let mut app = App::new(app, tx);

        let event_handler = async move {
            while let Ok(event) = rx.recv().await {
                match event {

                }
            }
        };

        utils::spawn(event_handler);
    });

    // This last step performs the same duty as gtk::main()
    app.run(&[]);
}
```

Calling `gtk::Application::new()` will run `gtk::init()` and register your application by the `APP_ID` that we defined. The general practice for application IDs is to use [Reverse domain name notation (RDNN)](https://en.wikipedia.org/wiki/Reverse_domain_name_notation). `gtk::Application::connect_activate()` signals that GTK is ready for us to construct our application window and set up all of our application logic. This method receives a reference to the `gtk::Application` itself, which we will later use to create the `gtk::ApplicationWindow`, which is our top level `gtk::Window` for our application. `gtk::Application::run()` will then invoke `gtk::main()` to set the whole process in motion.