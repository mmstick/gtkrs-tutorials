#[macro_use]
extern crate cascade;

mod app;
mod background;
mod widgets;
mod utils;

use std::path::PathBuf;
use self::app::App;
use self::background::BgEvent;
use gio::prelude::*;

pub const APP_ID: &str = "io.github.mmstick.ToDo";

slotmap::new_key_type!{
    pub struct TaskEntity;
}

pub enum Event {
    Delete,
    Insert(TaskEntity),
    Load(PathBuf, String),
    Modified,
    Remove(TaskEntity),
    SyncToDisk,
    Toggled(bool),
    Closed,
    Quit,
}

fn main() {
    let app_name = "Todo";

    glib::set_program_name(Some(app_name));
    glib::set_application_name(app_name);

    let app = gtk::Application::new(Some(APP_ID), Default::default())
        .expect("failed to init application");

    app.connect_activate(|app| {
        let (tx, rx) = async_channel::unbounded();
        let (btx, brx) = async_channel::unbounded();

        std::thread::spawn(glib::clone!(@strong tx => move || {
            utils::thread_context().block_on(background::run(tx, brx));
        }));

        let mut app = App::new(app, tx, btx);

        let event_handler = async move {
            while let Ok(event) = rx.recv().await {
                match event {
                    Event::Modified => app.modified(),
                    Event::Insert(entity) => app.insert(entity),
                    Event::Remove(entity) => app.remove(entity),
                    Event::SyncToDisk => app.sync_to_disk().await,
                    Event::Toggled(active) => app.toggled(active),
                    Event::Delete => app.delete(),
                    Event::Load(path, data) => app.load(path, data),
                    Event::Closed => app.closed().await,
                    Event::Quit => gtk::main_quit(),
                }
            }
        };

        utils::spawn(event_handler);
    });

    app.run(&[]);
}
