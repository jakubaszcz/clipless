mod database;
mod interface;
mod selected_text;
mod clipboard;

use std::default::Default;
use eframe::egui;
use eframe::egui::WindowLevel;
use eframe::epaint::Pos2;
use global_hotkey::GlobalHotKeyManager;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use crate::interface::MyApp;

fn main() -> eframe::Result<()> {

    // Initialize the database
    let connection = database::init_database().unwrap();

    // Initialize the hotkey manager
    let manager = GlobalHotKeyManager::new().unwrap();

    // Define the hotkeys for copying
    let custom_copy_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyC,
    );

    // Define the hotkeys for showing/hiding the window
    let custom_app_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyD,
    );

    // Get the hotkey IDs
    let copy_hot_key_id = custom_copy_hotkey.id();
    let app_hot_key_id = custom_app_hotkey.id();

    // Register the hotkeys
    {
        manager.register(custom_copy_hotkey).unwrap();
        manager.register(custom_app_hotkey).unwrap();
    }

    // Options for the window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_visible(false)
            .with_taskbar(false)
            .with_decorations(false),
        ..Default::default()
    };

    // Start listening for hotkey events & display the window
    {
        eframe::run_native(
            "Clipless",
            options,
            Box::new(|_cc| {
                Ok(Box::new(MyApp {
                    search_query: String::new(),
                    connection,
                    copy_hot_key_id,
                    app_hot_key_id,
                    clip_modal: None,
                }))
            }),
        )
    }
}