use eframe::{egui, App, Frame};
use eframe::egui::{Color32, Context, PopupCloseBehavior as OtherPopupCloseBehavior, ViewportCommand};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use rusqlite::Connection;
use crate::{database, selected_text};

// Define the app struct
pub(crate) struct MyApp {
    pub(crate) search_query: String,
    pub(crate) connection: Connection,
    pub(crate) copy_hot_key_id: u32,
    pub(crate) app_hot_key_id: u32,
    pub(crate) clip_modal: Option<u32>,
}

// Define the handle_input function
pub(crate) fn handle_input(app: &mut MyApp, ctx: &egui::Context) {
    let receiver = GlobalHotKeyEvent::receiver();

    while let Ok(_event) = receiver.try_recv() {
        if _event.state != HotKeyState::Released {
            continue;
        }

        if _event.id == app.copy_hot_key_id {
            let text = selected_text::select::get();

            if !text.trim().is_empty() {
                println!("Selected text: {}", text);

                if let Err(err) = database::insert_clip(&app.connection, &text) {
                    eprintln!("DB error: {}", err);
                }
            } else {
                println!("nothing selected");
            }
        }
        if _event.id == app.app_hot_key_id {
            ctx.send_viewport_cmd(ViewportCommand::Focus);
        }
    }
}

// Define the clip box function
fn clip_box(ui: &mut egui::Ui, ctx : &Context, id: u32, text: &str, connection: &Connection, clip_modal: &mut Option<u32>) {

    let max_text_length = 250;

    let display_text = if text.chars().count() > max_text_length {
        format!("{}...", text.chars().take(max_text_length).collect::<String>())
    } else {
        text.to_string()
    };

    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(6))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(display_text);

                ui.horizontal(|ui| {
                    if ui.button("Copy").clicked() {
                        ui.ctx().copy_text(text.to_string());
                    }
                    if ui.button("Delete").clicked() {
                        database::remove_clip(&connection, id).unwrap();
                    }

                    if text.chars().count() > max_text_length {
                        if ui.button("Expand").clicked() {
                            *clip_modal = Some(id)
                        }
                    }
                })
            });
        });

    // Modal box
    if *clip_modal == Some(id) {
        modal_box(ctx, id, text, connection, clip_modal);
    }
}

// Define the modal box function
fn modal_box(ctx: &Context, id: u32, text: &str, connection: &Connection ,clip_modal: &mut Option<u32>) {
    if egui::Modal::new(egui::Id::new("my_modal"))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(text);
                ui.horizontal(|ui| {
                    if ui.button("Copy").clicked() {
                        ui.ctx().copy_text(text.to_string());
                    }
                    if ui.button("Delete").clicked() {
                        database::remove_clip(&connection, id).unwrap();
                    }
                    if ui.button("Close").clicked() {
                        *clip_modal = None;
                    }})
            });
    }).should_close() { *clip_modal = None; }
}

// Implement the App trait
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _eframe: &mut Frame) {

        handle_input(self, ctx);

        ctx.send_viewport_cmd(ViewportCommand::Minimized(false));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Store Clipboard with CTRL + LALT + C");
            ui.label("Press CTRL + LALT + D to focus the window, do not close the app, if it happen it will shut down. Just turn the executable back.");
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query).hint_text("Search up a clip...")
                );
            });
            egui::ScrollArea::vertical().show(ui, |ui| {

                let query: Vec<(u32, String)>;

                // If the query is longer than 3 characters, fetch the clips from the database, otherwise get all clips
                if self.search_query.len() > 3 {
                    query = database::fetch_clips(&self.connection, &self.search_query).unwrap();
                } else {
                    query = database::get_clips(&self.connection).unwrap();
                }

                // If there are no clips, display a message
                if query.is_empty() {
                    ui.label("No clips found...");
                }

                // Display the clips
                for (id, content) in query {
                    clip_box(ui, ctx, id, &content, &self.connection, &mut self.clip_modal);
                }
            })
        });

        ctx.request_repaint();
    }
}

