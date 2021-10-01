# Using GLib as an Async Runtime

Before moving further, we should know that we can leverage the same async runtime that GTK uses for spawning and scheduling its tasks.

## Spawning tasks on the default executor

This will schedule our futures to execute on the main thread, alongside all of the futures scheduled by GTK itself.

```rust
use std::future::Future;

/// Spawns a task on the default executor and waits to receive its output
pub fn block_on<F>(future: F) -> F::Output where F: Future {
    glib::MainContext::default().block_on(future)
}

/// Spawns a task in the background on the default executor
pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static {
    glib::MainContext::default().spawn_local(future);
}
```

## Spawning tasks on the current thread's executor

Using this approach, you can spawn futures onto the executor that is registered to the thread you are blocking from. By default, there is no executor initialized for newly-spawned threads, so you'll have to create and assign them to the current thread when `glib::MainContext::thread_default()` returns `None`.

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

pub fn block_on<F>(future: F) -> F::Output where F: Future {
    thread_context().block_on(future)
}

pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static {
    thread_context().spawn_local(future);
}
```

## Spawning tasks on a background thread

This is useful if you spawn a background thread and want to execute your futures using a GLib executor on that thread.

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

pub fn block_on<F>(future: F) -> F::Output where F: Future {
    thread_context().block_on(future)
}

enum BackgroundEvent { DoThis }

fn main() {
    let (bg_tx, bg_rx) = async_channel::unbounded();

    std::thread::spawn(|| {
        block_on(async move {
            while let Ok(request) = bg_rx.recv().await {
                match request {
                    BackgroundEvent::DoThis => do_this().await,

                }
            }
        });
    });
}
```

## Spawning tasks on a thread pool

There is also an option of using `glib::ThreadPool`, which gives you exactly that. It defaults to the number of virtual CPU cores in the system, and by default parks threads that have been idle for more than 15 seconds. The pool does not have a requirement on mutability for spawning blocking tasks, so you can initialize this as a global variable using `once_cell::sync::Lazy` to use around your application, if you prefer this over a Rust-native thread pool like `rayon`. Perhaps to leverage system libraries.

```rust
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let pool = glib::ThreadPool::new_shared(None)
        .expect("failed to spawn thread pool");

    let _ = pool.push(|| {
        sleep(Duration::from_secs(1));
        println!("First Task");
    });

    let _ = pool.push(|| {
        sleep(Duration::from_secs(2));
        println!("Second Task");
    });

    let _ = pool.push(|| {
        sleep(Duration::from_secs(3));
        println!("Third Task");
    });

    // Wait for tasks to complete
    while pool.get_unprocessed() > 0 {
        sleep(Duration::from_secs(1));
    }
}
```

## Spawning futures on a glib::ThreadPool

However, this thread pool takes closures as inputs, rather than futures. So if you want to use for futures, you can combine it with the thread default executor above.

```rust
use async_io::Timer;
use std::time::Duration;
use std::thread::sleep;
use std::future::Future;

fn thread_context() -> glib::MainContext {
    glib::MainContext::thread_default()
        .unwrap_or_else(|| {
            let ctx = glib::MainContext::new();
            ctx.push_thread_default();
            ctx
        })
}

fn block_on<F>(future: F) -> F::Output where F: Future {
    thread_context().block_on(future)
}

fn main() {
    let pool = glib::ThreadPool::new_shared(None)
        .expect("failed to spawn thread pool");

    let _ = pool.push(|| {
        block_on(async {
            Timer::after(Duration::from_secs(1)).await;
            println!("First Task");
        })
    });

    let _ = pool.push(|| {
        block_on(async {
            Timer::after(Duration::from_secs(2)).await;
            println!("Second Task");
        })
    });

    let _ = pool.push(|| {
        block_on(async {
            Timer::after(Duration::from_secs(3)).await;
            println!("Third Task");
        })
    });

    // Wait for tasks to complete
    block_on(async {
        while pool.get_unprocessed() > 0 {
            Timer::after(Duration::from_secs(1)).await;
        }
    })
}
```