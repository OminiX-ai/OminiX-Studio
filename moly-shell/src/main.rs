mod app;

/// Sets the macOS Dock icon using the bundled .icns file.
/// This is needed when running via `cargo run` since the binary isn't inside
/// the .app bundle and macOS can't read CFBundleIconName from Info.plist.
#[cfg(target_os = "macos")]
fn set_dock_icon() {
    std::thread::spawn(|| {
        // Wait for NSApplication run loop to be ready
        std::thread::sleep(std::time::Duration::from_millis(200));

        let icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/AppIcon.icns");

        unsafe {
            use objc::runtime::{Class, Object};
            use objc::{msg_send, sel, sel_impl};

            let path_bytes = std::ffi::CString::new(icon_path).unwrap();
            let ns_string_cls = Class::get("NSString").unwrap();
            let ns_path: *mut Object = msg_send![
                ns_string_cls,
                stringWithUTF8String: path_bytes.as_ptr()
            ];

            let ns_image_cls = Class::get("NSImage").unwrap();
            let image: *mut Object = msg_send![ns_image_cls, alloc];
            let image: *mut Object = msg_send![image, initWithContentsOfFile: ns_path];

            if !image.is_null() {
                let app_cls = Class::get("NSApplication").unwrap();
                let app: *mut Object = msg_send![app_cls, sharedApplication];
                let _: () = msg_send![app, setApplicationIconImage: image];
            }
        }
    });
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Set working directory to the executable's directory
        // This is critical for macOS app bundles to find resources in Contents/Resources/
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                let _ = std::env::set_current_dir(exe_dir);
            }
        }
    }

    // Initialize the logger
    env_logger::init();
    log::info!("Starting Moly");

    // Set Dock icon after the run loop starts (needed for cargo run)
    #[cfg(target_os = "macos")]
    set_dock_icon();

    // macOS 26 requires setActivationPolicy to be called before the event loop
    // starts, otherwise NSAssertMainEventQueueIsCurrentEventQueue fires on the
    // first nextEventMatchingMask call.
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::runtime::Class;
        use objc::{msg_send, sel, sel_impl};
        if let Some(ns_app_cls) = Class::get("NSApplication") {
            let ns_app: *mut objc::runtime::Object = msg_send![ns_app_cls, sharedApplication];
            let () = msg_send![ns_app, setActivationPolicy: 0i64]; // NSApplicationActivationPolicyRegular
        }
    }

    app::app_main();
}
