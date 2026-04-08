mod database;
use eframe::{egui, App, Frame};
use rusqlite::Connection;
use egui::ViewportCommand;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use selection::get_text;
struct MyApp {
    connection: Connection,
    global_hot_key: GlobalHotKeyManager,
    copy_hot_key_id: u32,
    app_hot_key_id: u32,
    window_visibility: bool,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _eframe: &mut Frame) {

        let receiver = GlobalHotKeyEvent::receiver();

        while let Ok(_event) = receiver.try_recv() {
            if _event.state != HotKeyState::Released {
                continue;
            }

            if _event.id == self.copy_hot_key_id {
                let text = get_text();

                if !text.trim().is_empty() {
                    println!("Selected text: {}", text);

                    if let Err(err) = database::insert_clip(&self.connection, &text) {
                        eprintln!("DB error: {}", err);
                    }
                } else {
                    println!("nothing selected");
                }
            }
            if _event.id == self.app_hot_key_id {
                self.window_visibility = !self.window_visibility;

                if self.window_visibility {
                    ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
                    ctx.send_viewport_cmd(ViewportCommand::Focus);
                    println!("window shown");
                } else {
                    ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                    println!("window hidden");
                }
            }
        }

        ctx.send_viewport_cmd(ViewportCommand::Minimized(false));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clipless");
            ui.label("Ctrl + Alt + C : save selected text");
            ui.label("Ctrl + Alt + D : toggle window");
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let connection = database::init_database().unwrap();
    let manager = GlobalHotKeyManager::new().unwrap();


    let custom_copy_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyC,
    );

    let custom_app_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyD,
    );

    manager.register(custom_copy_hotkey).unwrap();
    manager.register(custom_app_hotkey).unwrap();

    let copy_hot_key_id = custom_copy_hotkey.id();
    let app_hot_key_id = custom_app_hotkey.id();

    eframe::run_native(
        "Clipless",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                connection,
                global_hot_key: manager,
                copy_hot_key_id,
                app_hot_key_id,
                window_visibility: false,
            }))
        }),
    )
}