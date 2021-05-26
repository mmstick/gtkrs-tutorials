# Creating a Window with a Button

Let's start by setting up a convenience function for spawning futures on the default executor. This will be necessary to send messages through the async channel.

```rust
use std::future::Future;

/// Spawns a task on the default executor, without waiting for it to complete
pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static {
    glib::MainContext::default().spawn_local(future);
}
```

## Creating the App struct

I typically have a single `App` struct where all application state and GTK widgets that are regularly interacted with are stored. We're going to start with a struct that contains a `gtk::Button` and a `u32` "clicked" variable.

```rust
use async_channel::Sender;

struct App {
    pub button: gtk::Button,
    pub clicked: u32,
}

impl App {
    pub fn new(tx: Sender<Event>) -> Self {}
}
```

When creating the application, we will take ownership of the `Sender` that we created earlier, and pass this into every `.connect_signal()` method that is called on a widget. The `.connect_signal()` methods will create a future on the main context that idles until the condition for that future has been emitted. A `gtk::Button`, for example, has a `.connect_clicked()` method which will have its callbacks invoked when `clicked` is emitted — which happens on a click of the button.

Note that you may connect multiple callbacks onto the same signal. If you wish to remove one, you should be careful to store the `SignalHandlerId` that is returned from the `.connect_signal()` method. Then call `widget.disconnect(id)` to remove the signal registered to that widget. If you only wish to temporarily block a signal, you can call `widget.block_signal(id)` and `widget.unblock_signal(id)` respectively.

## Creating widgets for our app

First, we will create the button that we will have the user click. The button will have a label which reads, "Click Me". The border will be set to 4 so that the button isn't hugging the edges of the container it is attached to. And then will program it to send an event when it is clicked.

```rust
let button = cascade! {
    gtk::Button::with_label("Click Me");
    ..set_border_width(4);
    ..connect_clicked(move |_| {
        let tx = tx.clone();
        spawn(async move {
            let _ = tx.send(Event::Clicked).await;
        });
    });
};
```

Note that since we are using an async channel, the sender has to be awaited when it is sending a value. We can use GLib's default executor to await our send.

> If the sender happens to block, it could block the default executor and thereby freeze the application. If you're using an unbounded receiver, it will never block on a send, so you will not have to worry about this.
>
> When using a bounded receiver, you should ensure that the tasks is spawned on the executor so that at least the sender can safely wait for its turn to send without blocking our application. However, there is no reason to use a bounded receiver for receiving events, because you'll simply cause the executor to fill up with unresolved tasks.

Next is creating a container widget to hold our button. This container will also invoke `.show_all()` to make the container visible, and all of the widgets inside the container.

```rust
let container = cascade! {
    gtk::Box::new(gtk::Orientation::Vertical, 0);
    ..add(&button);
    ..show_all();
};
```

## Creating the window

Next we we will create the `Toplevel` window for this application, and attach our container to the window. We will set a title, connect the event to be called when window is deleted, and also set the default icon for our application. The `Toplevel` window is the main window of your application. A window can only have one widget attached to it, which we will assign with the `.add()` method. The `.set_title()` method will set the title of your application. The `.connect_delete_event` method is invoked whenever the window is destroyed, and we will program this to call `gtk::main_quit()` to stop the mainloop, thereby having `gtk::main()` return, which has our application quit.


```rust
let _window = cascade! {
    gtk::Window::new(gtk::WindowType::Toplevel);
    ..add(&container);
    ..set_title("First GTK App");
    ..set_default_size(300, 400);
    ..connect_delete_event(move |_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    ..show_all();
};
```

One last thing that should be done for window managers is to set a default icon for the application:

```rust
gtk::Window::set_default_icon_name("icon-name-here");
```

Now we can finally return our `App` struct, which should look like so:


```rust
impl App {
    pub fn new(tx: Sender<Event>) -> Self {
        let button = cascade! {
            gtk::Button::with_label("Click Me");
            ..set_border_width(4);
            ..connect_clicked(move |_| {
                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Clicked).await;
                });
            });
        };

        let container = cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 0);
            ..add(&button);
            ..show_all();
        };

        let _window = cascade! {
            gtk::Window::new(gtk::WindowType::Toplevel);
            ..set_title("First GTK App");
            ..add(&container);
            ..connect_delete_event(move |_, _| {
                gtk::main_quit();
                gtk::Inhibit(false)
            });
            ..show_all();
        };

        gtk::Window::set_default_icon_name("icon-name-here");

        Self { button, clicked: 0 }
    }
}
```

## Responding to the clicked event

In the example below, you can see that we have passed ownership of the `App` into the event handler. The `clicked` property is incremented whenever we receive `Event::Clicked`. The button's label is updated to show how many times it has been clicked.

```rust
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

    let mut app = App::new(tx);

    // Processes all application events received from signals
    let event_handler = async move {
        while let Ok(event) = rx.recv().await {
            match event {
                Event::Clicked => {
                    app.clicked += 1;
                    app.button.set_label(&format!("I have been clicked {} times", app.clicked));
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

You may run the application with `cargo run` and try it out.