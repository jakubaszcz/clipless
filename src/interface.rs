use eframe::{egui, App, Frame};
use eframe::egui::{Color32, Context, PopupCloseBehavior as OtherPopupCloseBehavior, ViewportCommand};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use rusqlite::Connection;
use crate::{database, selected_text};

// Define the app struct
pub(crate) struct MyApp {
    pub(crate) connection: Connection,
    pub(crate) copy_hot_key_id: u32,
    pub(crate) app_hot_key_id: u32,
    pub(crate) window_visibility: bool,
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
            app.window_visibility = !app.window_visibility;

            if app.window_visibility {
                ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
                ctx.send_viewport_cmd(ViewportCommand::Focus);
                println!("window shown");
            } else {
                ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                println!("window hidden");
            }
        }
    }
}

// Define the clip box function
fn clip_box(ui: &mut egui::Ui, ctx : &Context, id: u32, text: &str, connection: &Connection) {

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
                })
            });
        });
}

// Implement the App trait
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _eframe: &mut Frame) {

        handle_input(self, ctx);

        ctx.send_viewport_cmd(ViewportCommand::Minimized(false));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Store Clipboard with CTRL + LALT + C");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Ok(clips) = database::get_clips(&self.connection) {
                    for (id, content) in clips {
                        clip_box(ui, ctx, id, &content, &self.connection);
                    }
                }
            })
        });

        ctx.request_repaint();
    }
}

