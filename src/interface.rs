use eframe::{egui, App, Frame};
use eframe::egui::{Context, ViewportCommand};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use rusqlite::Connection;
use chrono::{Utc, DateTime};
use crate::{clipboard, database, selected_text};

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
fn clip_box(ui: &mut egui::Ui, ctx : &Context, clip: clipboard::Clipboard, connection: &Connection, clip_modal: &mut Option<u32>) {
    let max_text_length = 250;

    let display_text = if clip.content.chars().count() > max_text_length {
        format!("{}...", clip.content.chars().take(max_text_length).collect::<String>())
    } else {
        clip.content.to_string()
    };

    egui::Frame::new()
        .fill(egui::Color32::from_hex("#F2EFE9").unwrap())
        .corner_radius(2.0)
        .show(ui, |ui| {
            egui::Frame::new()
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.set_min_width(ui.available_width());

                        ui.label(
                            egui::RichText::new(display_text)
                                .color(egui::Color32::from_hex("#252627").unwrap())
                        );
                        ui.horizontal(|ui| {
                            if ui.add(
                                egui::Button::new(
                                    egui::RichText::new("Copy")
                                        .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                        .strong()
                                )
                                    .fill(egui::Color32::from_hex("#252627").unwrap())
                                    .corner_radius(egui::CornerRadius::same(2))
                            ).clicked() {
                                ui.ctx().copy_text(clip.content.to_string());
                                database::update_use_clip(connection, clip.id).unwrap();
                            }
                            if ui.add(
                                egui::Button::new(
                                    egui::RichText::new("Delete")
                                        .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                        .strong()
                                )
                                    .fill(egui::Color32::from_hex("#252627").unwrap())
                                    .corner_radius(egui::CornerRadius::same(2))
                            ).clicked() {
                                database::remove_clip(connection, clip.id).unwrap();
                            }
                            if clip.content.chars().count() > max_text_length {
                                if ui.add(
                                    egui::Button::new(
                                        egui::RichText::new("Expand")
                                            .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                            .strong()
                                    )
                                        .fill(egui::Color32::from_hex("#252627").unwrap())
                                        .corner_radius(egui::CornerRadius::same(2))
                                ).clicked() {
                                    *clip_modal = Some(clip.id);
                                }
                            }

                            ui.label(
                                egui::RichText::new(DateTime::<Utc>::from_timestamp(clip.timestamp, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string())
                                    .color(egui::Color32::from_hex("#252627").unwrap())
                            );
                            if clip.use_clip > 0 {
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new(format!("It has been copied {} times", clip.use_clip))
                                        .color(egui::Color32::from_hex("#252627").unwrap())
                                );
                            }
                        })
                })
        });
    });

    // Modal box
    if *clip_modal == Some(clip.id) {
        modal_box(ctx, clip, connection, clip_modal);
    }
}

// Define the modal box function
fn modal_box(ctx: &Context, clip: clipboard::Clipboard, connection: &Connection ,clip_modal: &mut Option<u32>) {
    if egui::Modal::new(egui::Id::new("my_modal"))
        .frame(
            egui::Frame::new()
                .corner_radius(2.0)
                .inner_margin(egui::Margin::same(6))
                .fill(egui::Color32::from_hex("#F2EFE9").unwrap()))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(clip.content.to_string());
                ui.horizontal(|ui| {
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("Copy")
                                .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                .strong()
                        )
                            .fill(egui::Color32::from_hex("#252627").unwrap())
                            .corner_radius(egui::CornerRadius::same(2))
                    ).clicked() {
                        ui.ctx().copy_text(clip.content.to_string());
                        database::update_use_clip(connection, clip.id).unwrap();
                    }
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("Delete")
                                .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                .strong()
                        )
                            .fill(egui::Color32::from_hex("#252627").unwrap())
                            .corner_radius(egui::CornerRadius::same(2))
                    ).clicked() {
                        database::remove_clip(connection, clip.id).unwrap();
                    }
                    ui.label(
                        egui::RichText::new(DateTime::<Utc>::from_timestamp(clip.timestamp, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string())
                            .color(egui::Color32::from_hex("#252627").unwrap())
                    );
                    if clip.use_clip > 0 {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!("It has been copied {} times", clip.use_clip))
                                .color(egui::Color32::from_hex("#252627").unwrap())
                        );
                    }
                })
            });
    }).should_close() { *clip_modal = None; }
}

// Implement the App trait
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _eframe: &mut Frame) {

        handle_input(self, ctx);

        ctx.send_viewport_cmd(ViewportCommand::Minimized(false));

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#252627").unwrap())
            )
            .show(ctx, |ui| {

                // Header
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Clipless")
                                    .size(20.0)
                                    .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                            );
                            ui.add_space(10.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut self.search_query).hint_text("Search up a clip...")
                                    .background_color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                    .text_color(egui::Color32::from_hex("#252627").unwrap())
                                    .lock_focus(false)
                            );
                        })
                    });

                // Content
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {

                            let query: Vec<clipboard::Clipboard>;

                            // If the query is longer than 3 characters, fetch the clips from the database, otherwise get all clips
                            if self.search_query.len() > 3 {
                                query = database::fetch_clips(&self.connection, &self.search_query).unwrap();
                            } else {
                                query = database::get_clips(&self.connection).unwrap();
                            }

                            // If there are no clips, display a message
                            let empty = egui::Frame::new();

                            if query.is_empty() {
                                empty.show(ui, |ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(
                                            egui::RichText::new("Nothing to be found...")
                                                .color(egui::Color32::from_hex("#F2EFE9").unwrap())
                                        );
                                    })
                                });
                                return;
                            }

                            // Display the clips
                            for clips in query {
                                clip_box(ui, ctx, clips, &self.connection, &mut self.clip_modal);
                            }
                        })
                    });
        });

        ctx.request_repaint();
    }
}

