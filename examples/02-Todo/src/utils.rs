use std::future::Future;

pub fn thread_context() -> glib::MainContext {
    glib::MainContext::thread_default().unwrap_or_else(|| {
        let ctx = glib::MainContext::new();
        ctx.push_thread_default();
        ctx
    })
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    glib::MainContext::default().spawn_local(future);
}