use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let manager = GlobalHotKeyManager::new().unwrap();

    let custom_copy_hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyC,
    );

    manager.register(custom_copy_hotkey).unwrap();

    let receiver = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _| {
        if let Ok(_event) = receiver.try_recv() {
            println!("{:?}", _event);
        }
    }).unwrap();
}