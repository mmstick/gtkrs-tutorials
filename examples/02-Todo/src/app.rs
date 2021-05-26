use crate::{Event, BgEvent, TaskEntity};
use crate::widgets::Task;
use crate::utils::spawn;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;
use std::path::PathBuf;

pub struct App {
    pub container: gtk::Grid,
    pub delete_button: gtk::Button,
    pub headerbar: gtk::HeaderBar,
    pub tasks: SlotMap<TaskEntity, Task>,
    pub scheduled_write: Option<SourceId>,
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
    pub checks_active: u32,
    pub last_saved: PathBuf,
}

impl App {
    pub fn new(
        app: &gtk::Application,
        tx: Sender<Event>,
        btx: Sender<BgEvent>,
    ) -> Self {
        let container = cascade! {
            gtk::Grid::new();
            ..set_column_spacing(4);
            ..set_row_spacing(4);
            ..set_border_width(4);
            ..show();
        };

        let scrolled = gtk::ScrolledWindowBuilder::new()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();

        scrolled.add(&container);

        let delete_button = cascade! {
            gtk::Button::from_icon_name(Some("edit-delete-symbolic"), gtk::IconSize::Button);
            ..set_label("Delete");
            // Show the icon alongside the label
            ..set_always_show_image(true);
            // Don't show this when the window calls `.show_all()`
            ..set_no_show_all(true);
            // Give this a destructive styling to signal that the action is destructive
            ..get_style_context().add_class(&gtk::STYLE_CLASS_DESTRUCTIVE_ACTION);
            // Send the `Delete` event on click
            ..connect_clicked(clone!(@strong tx => move |_| {
                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Delete).await;
                });
            }));
        };

        let data_dir = xdg::BaseDirectories::with_prefix(crate::APP_ID)
            .unwrap()
            .get_data_home();

        let last_saved = data_dir.join("Default");

        let open_button = cascade! {
            gtk::Button::from_icon_name(Some(""), gtk::IconSize::Button);
            ..set_label("Open");
            ..set_always_show_image(true);
            ..connect_clicked(clone!(@strong btx => move |_| {
                let dialog = gtk::FileChooserNative::new(
                    Some("Choose a Note"),
                    Some(&gtk::Window::new(gtk::WindowType::Popup)),
                    gtk::FileChooserAction::Save,
                    Some("_Open"),
                    Some("_Cancel")
                );

                dialog.set_current_folder(&data_dir);

                if let gtk::ResponseType::Accept = dialog.run() {
                    if let Some(path) = dialog.get_filename() {
                        let btx = btx.clone();
                        spawn(async move {
                            let _ = btx.send(BgEvent::Load(path)).await;
                        });
                    }
                }

                dialog.destroy();
            }));
        };

        let headerbar = cascade! {
            gtk::HeaderBar::new();
            ..pack_end(&open_button);
            ..pack_end(&delete_button);
            ..set_title(Some("ToDo"));
            ..set_subtitle(Some("Default"));
            ..set_show_close_button(true);
        };

        let _window = cascade! {
            gtk::ApplicationWindow::new(app);
            ..set_titlebar(Some(&headerbar));
            ..set_default_size(400, 600);
            ..add(&scrolled);
            ..connect_delete_event(clone!(@strong tx, @strong scrolled => move |win, _| {
                // Detach to preserve widgets after destruction of window
                win.remove(&scrolled);

                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Closed).await;
                });
                gtk::Inhibit(false)
            }));
            ..show_all();
        };

        gtk::Window::set_default_icon_name("icon-name-here");

        let mut app = Self {
            container,
            delete_button,
            tasks: SlotMap::with_key(),
            scheduled_write: None,
            tx,
            btx,
            checks_active: 0,
            last_saved,
            headerbar
        };

        app.insert_row(0);

        app
    }

    pub fn clear(&mut self) {
        while let Some(entity) = self.tasks.keys().next() {
            self.remove_(entity);
        }
    }

    pub fn delete(&mut self) {
        let remove_list = self.tasks.iter()
            .filter(|(_, task)| task.check.get_active())
            .map(|(id, _)| id)
            .collect::<Vec<TaskEntity>>();

        for id in remove_list {
            self.remove(id);
        }

        self.checks_active = 0;
        self.delete_button.set_visible(false);
    }

    pub fn toggled(&mut self, active: bool) {
        if active {
            self.checks_active += 1;
        } else {
            self.checks_active -= 1;
        }

        self.delete_button.set_visible(self.checks_active != 0);
    }

    pub fn insert(&mut self, entity: TaskEntity) {
        let mut insert_at = 0;

        if let Some(task) = self.tasks.get(entity) {
            insert_at = task.row + 1;
        }

        self.insert_row(insert_at);
    }

    fn insert_row(&mut self, row: i32) -> TaskEntity {
        // Increment the row value of each Task is below the new row
        for task in self.tasks.values_mut() {
            if task.row >= row {
                task.row += 1;
            }
        }

        self.container.insert_row(row);
        let task = Task::new(row);

        self.container.attach(&task.check, 0, row, 1, 1);
        self.container.attach(&task.entry, 1, row, 1, 1);
        self.container.attach(&task.insert, 2, row, 1, 1);

        task.entry.grab_focus();

        let entity = self.tasks.insert(task);
        self.tasks[entity].connect(self.tx.clone(), entity);
        return entity;
    }

    pub fn load(&mut self, path: PathBuf, data: String) {
        self.clear();

        for (row, line) in data.lines().enumerate() {
            let entity = self.insert_row(row as i32);
            self.tasks[entity].set_text(line);
        }

        use std::ffi::OsStr;
        self.headerbar.set_subtitle(path.file_name().and_then(OsStr::to_str));
        self.last_saved = path;
    }

    pub fn modified(&mut self) {
        if let Some(id) = self.scheduled_write.take() {
            glib::source_remove(id);
        }

        let tx = self.tx.clone();
        self.scheduled_write = Some(glib::timeout_add_local(5000, move || {
            let tx = tx.clone();
            spawn(async move {
                let _ = tx.send(Event::SyncToDisk).await;
            });

            glib::Continue(false)
        }));
    }

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

    pub async fn closed(&mut self) {
        self.sync_to_disk().await;
        let _ = self.btx.send(BgEvent::Quit).await;
    }

    pub async fn sync_to_disk(&mut self) {
        self.scheduled_write = None;

        let contents = fomat_macros::fomat!(
            for node in self.tasks.values() {
                if node.entry.get_text_length() != 0 {
                    (node.entry.get_text()) "\n"
                }
            }
        );

        let _ = self.btx.send(BgEvent::Save(self.last_saved.clone(), contents)).await;
    }
}
