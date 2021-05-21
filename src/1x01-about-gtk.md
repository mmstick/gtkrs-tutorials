# About GTK

> **WARNING**: This tutorial assumes familiarity with Rust.

Before we begin, it is important to know a few things about GTK itself. The architecture that GTK is built upon strongly influences the way that we will interact with it. Yet I won't dive too deeply into the details, because we only need cover what's most important for us as a consumer of the API in Rust.

## GTK is built around GLib

GTK is a GUI toolkit built strongly around GLib and it's GLib Object System — which we'll simply call GObject. GLib is essentially a high level cross-platform standard library for C. The GObject portion of GLib enables programming with an object-oriented paradigm.

GTK uses GLib both for it's object system and asynchronous runtime. Every widget type is a GObject class, and most widget classes inherit multiple layers of GTK widget classes. Widgets schedule tasks for concurrent execution on the default async executor (`gtk::MainContext::default()`), and register signals that react to various property and state changes.

Luckily for us, the behaviors of every class implemented in GTK can be conveniently represented as a trait in Rust. Even better, GTK fully supports a feature known as "GObject Introspection", which is a convenient way of automatically generating quality bindings in other programming languages. This allowed GTK to have first class bindings in a short amount of time.

## Initialized widgets are objects

As for what that means to us, `GObjects` are heap-allocated with interior mutability. Behaviorally, they operate very similarly to how you would expect to work with a `Rc<RefCell<T>>` type in Rust. You'll never be required to have unique ownership or a mutable reference to modify a widget, as a result.

When you clone a GObject, you are creating a new strong reference to the object. When all strong references have been dropped, the destructor for that object is run. However, cyclic references are possible with strong references which can prevent the strong count from ever reaching 0, so there's an option to downgrade them into a weak reference. We'll be designing our software in a way that mitigates the need for this though.

## Widgets have inheritance

Being built in an object-oriented fashion, widgets are built by inheriting other widgets. So a `gtk::Box` is a subclass of `gtk::Container`, which is also a subclass of `gtk::Widget`; and therefore we can downgrade a `gtk::Box` into a `gtk::Container`, and we can further downgrade that into a `gtk::Widget`.

There may be times when an API hands you a `gtk::Widget`, and you'll need to upgrade that widget into a more specific type of widget if you want to access methods from that widget's class.

Or there may also be times when you just want to simplify your code and downgrade a widget into a `gtk::Widget` because you're passing it onto something that takes any kind of widget as an input.

## Widget classes have traits

In the GTK-rs implementation, methods from classes are conveniently stored in traits. The `gtk::Widget`, `gtk::Container`, and `gtk::Box` classes have their methods stored in their respective `gtk::WidgetExt`, `gtk::ContainerExt`, and `gtk::BoxExt` traits. This will allow you to conveniently handle your widgets in a generic fashion. Maybe you have a function that can be perfectly written as `fn function<W: WidgetExt>(widget: &W) {}`,

## GTK is not thread-safe

Finally, although GObjects can be thread-safe, GTK widgets are most definitely not. You should not attempt to send GTK widgets across thread boundaries, which thankfully the Rust type system will not permit. GTK widgets must be created and interacted with exclusively on the thread that GTK was initialized on.

There are crates such as `fragile` that would allow you to wrap your widgets into a `Fragile<T>` and send them across threads, but this is most certainly an anti-pattern. The way a `Fragile<T>` works is that it prevents you from accessing the `T` inside the wrapper unless you are unwrapping it from the same thread that it was created on. If you design your software correctly, you won't have to resort to this kind of arcane magic. Turn back before it is too late.