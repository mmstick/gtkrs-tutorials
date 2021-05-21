# Getting Started

## Dependencies

Before we begin, ensure that you have the necessary development files installed for GTK. On Debian platforms, you will need:

- `libgtk-3-dev` for GTK3
- `libgtk-4-dev` for GTK4
- `libwebkit2gtk-4.0-dev` if embedding a GTK WebKit View
- `libgtksourceview-4-dev` if embedding a GTK Source View

On the Rust side of things, you should install:

-  `cargo-edit` with `cargo install cargo-edit`, because that'll make adding dependencies to your project easier.
-  `rust-analyzer` in your IDE so that you'll have instant feedback about warnings and errors as you type


## API Documentation

The API documentation generated on [docs.rs](https://docs.rs) lacks descriptions of the APIs. If you want the most complete API documentation, you will need to reference the documentation generated on the gtk-rs website [here](https://gtk-rs.org/docs-src/).

To navigate this API, every widget has its own type, but those types only contain methods for constructing the widget. Methods specific to each widget can be found in the `Ext` trait for that widget, such as `ButtonExt`. You may reference the widget type to see what behaviors it implements, such as `ContainerExt` or `WidgetExt`.

Finally, each widget also has a `Builder` type, such as `ButtonBuilder`. In some cases, the builder type will be the only way to achieve a certain desired effect, such creating a dialog with a `gtk::HeaderBar`. This is because this method will define each property before the widget is initialized.

## Cascade Macro

This macro is an alternative to the builder pattern that I find more useful in general, as the builder type only works up to creation of the widget, rather than calling methods on created widget itself — such as adding widgets to a container.

```rust
let container = cascade! {
    gtk::Box::new(gtk::Orientation::Vertical, 0);
    ..add(&widget1);
    ..add(&widget2);
};
```

Essentially, the first statement creates your widget, and following lines that start with `.` will allow you to invoke a method on that widget, before finally returning the widget itself.

## Creating Your Project

Now we're going to start the process of actually building our first GTK application. Create your project, and add the following dependencies that we need to get started:

```
cargo new first-gtk-app
cd first-gtk-app
cargo add gtk glib async-channel cascade
```

### Initializing GTK

Now we're ready to start with code. Lets start by setting your application's name, and initializing GTK.

```rust
#[macro_use]
extern crate cascade;

use gtk::prelude::*;
use std::process;

fn main() {
    glib::set_program_name("First GTK App".into());
    glib::set_application_name("First GTK App");

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Thread will block here until the application is quit
    gtk::main();
}
```