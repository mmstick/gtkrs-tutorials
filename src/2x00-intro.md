# ToDo

> Full source code for this chapter can be found in the [GitHub Repository](https://github.com/mmstick/gtkrs-tutorials) under `examples/02-Todo`.

The first application that we will create in this tutorial series is a ToDo list. There are a myriad of ways to create them, but we are going to opt for the event-driven approach with a [slotmap](https://docs.rs/slotmap). Each task in the ToDo list will be stored in the SlotMap and referenced by its key. On launch of the application, we will load the most-recently modified note. On close, we will write any changes that have yet to be saved before quitting the application. We will also save any changes made every 5 seconds after the last modification.

## What is a SlotMap?

SlotMap is described as a "container with persistent unique keys to access stored values." Essentially, it is an arena allocator with generational indices. Imagine a vector where each slot contains a version number. Odd-numbered versions indicate to the allocator that the slot is empty and ready to be filled. On insertion of a new value into a slot, the version is incremented back to an even number, and a key is returned which contains the "generation" version, and the indice of the slot that was used. This gives SlotMap roughly the same performance as accessing an element of an array by its indice, but with an additional version check to determine if key is still valid.

## Initialize the project

To get started, create a new project:

```
cargo new todo
cd todo
cargo add async-channel cascade fomat-macros gio glib gtk slotmap xdg
```

Then inside the src folder, structure your project like so:

```
src/
    app.rs
    background.rs
    main.rs
    utils.rs
    widgets.rs
```
