mod app;
mod apps;

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

    app::app_main();
}
