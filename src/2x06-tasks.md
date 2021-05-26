# Inserting and Removing Tasks

## app.rs

### Inserting a Row

Back to our `App` struct, we're going to work on the ability to insert a row by the row indice.

```rust
fn insert_row(&mut self, row: i32) -> TaskEntity {

}
```

When inserting a row, we will want to increment the row value of each task is below the row being added. We can achieve that by iterating our SlotMap of tasks by value, mutably. task that has a row that is greater or equal to the row being inserted will be incremented by 1.

```rust
// Increment the row value of each Task is below the new row
for task in self.tasks.values_mut() {
    if task.row >= row {
        task.row += 1;
    }
}
```

Then we instruct our `gtk::Grid` to insert this new row, pushing down all rows beneath it:

```rust
self.container.insert_row(row);
```

Next we'll create our task widgets, and assign them to the grid. The `.attach()` method takes the widget to assign, followed by the column, row, width, and height parameters.

```rust
let task = Task::new(row);

self.container.attach(&task.entry, 0, row, 1, 1);
self.container.attach(&task.insert, 1, row, 1, 1);
self.container.attach(&task.remove, 2, row, 1, 1);
```

We should also ensure that the newly-added `gtk::Entry` will grab the focus of our keyboard

```rust
task.entry.grab_focus();
```

Now we can assign this newly-created `Task` to our SlotMap. This will return a key, which we will use as identifiers to the signals we're now going to connect.

```rust
let entity = self.tasks.insert(task);
self.tasks[entity].connect(self.tx.clone(), entity);
return entity;
```

Your method should now look like this:

```rust
fn insert_row(&mut self, row: i32) -> TaskEntity {
    // Increment the row value of each Task is below the new row
    for task in self.tasks.values_mut() {
        if task.row >= row {
            task.row += 1;
        }
    }

    self.container.insert_row(row);
    let task = Task::new(row);

    self.container.attach(&task.entry, 0, row, 1, 1);
    self.container.attach(&task.insert, 1, row, 1, 1);
    self.container.attach(&task.remove, 2, row, 1, 1);

    task.entry.grab_focus();

    let entity = self.tasks.insert(task);
    self.tasks[entity].connect(self.tx.clone(), entity);
    return entity;
}
```

## widgets.rs

It is at this point where we are going to start connecting the signals to our task widgets. Add the following method to your `Task` struct:

```rust
pub fn connect(&mut self, tx: Sender<Event>, entity: TaskEntity) {

}

```

First we will have the entry send `Event::Modified` whenever it has changed:

```rust
self.entry.connect_changed(clone!(@strong tx => move |_| {
    let tx = tx.clone();
    spawn(async move {
        let _ = tx.send(Event::Modified).await;
    });
}));
```

Then we will program insert button to send `Event::Insert(entity)` when it has been clicked. Although we will only send this signal if the entry for this task is empty. Note that we are taking the entry widget by weak reference. This will prevent a potential cyclic reference when two widgets happen to depend on each other in their signals. The `clone!` macro will automatically handle creating the weak reference, and upgrading that reference in our signal.

```rust
self.insert
    .connect_clicked(clone!(@strong tx, @weak self.entry as entry => move |_| {
        if entry.get_text_length() == 0 {
            return;
        }

        let tx = tx.clone();
        spawn(async move {
            let _ = tx.send(Event::Insert(entity)).await;
        });
    }));
```

Then the remove button:

```rust
self.remove.connect_clicked(clone!(@strong tx => move |_| {
    let tx = tx.clone();
    spawn(async move {
        let _ = tx.send(Event::Remove(entity)).await;
    });
}));
```

And to respond to when the user presses the Enter key, which should be treated as equivalent to clicking the insert button:

```rust
    self.entry
        .connect_activate(clone!(@weak self.entry as entry => move |_| {
            if entry.get_text_length() == 0 {
                return;
            }

            let tx = tx.clone();
            spawn(async move {
                let _ = tx.send(Event::Insert(entity)).await;
            });
        }));
}
```


## app.rs

Moving back to our app module, we'll add another method for inserting a row. Because our application is going to insert new rows from received events via the `TaskEntity` that was received in the `Insert(TaskEntity)` event, we need to add the method that our application is going to call. After fetching the task from the SlotMap, we can use the conveniently-stored `row` value to determine where we're going to insert a new row.

```rust
pub fn insert(&mut self, entity: TaskEntity) {
    let mut insert_at = 0;

    if let Some(task) = self.tasks.get(entity) {
        insert_at = task.row + 1;
    }

    self.insert_row(insert_at);
}
```

Finally, we get to removing tasks. When we receive that `Remove(TaskEntity)` event, we're going to call `app.remove(entity)`. We'll ignore any requests to delete the last task from the list, since that would render our application unusable. If we're allowed to remove a task, we'll remove the task from the SlotMap, and call `grid.remove_row(&widget)` on our `container` to remove all the widgets from that task's row from the container. The widgets will be automatically destroyed after returning from this function, because the last remaining strong references to them have been wiped out.

```rust
pub fn remove(&mut self, entity: TaskEntity) {
    if self.tasks.len() == 1 {
        return;
    }
    self.remove_(entity);
}

fn remove_(&mut self, entity: TaskEntity) {
    if let Some(removed) = self.tasks.remove(entity) {
        self.container.remove_row(removed.row);

        // Decrement the row value of the tasks that were below the removed row
        for task in self.tasks.values_mut() {
            if task.row > removed.row {
                task.row -= 1;
            }
        }
    }
}
```

And of course, similar to having to increment the row values on insert, we'll do the reverse on removal of a widget.