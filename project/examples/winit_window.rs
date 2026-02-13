fn main() {
    let app = evo_ui_engine::runtime::TomlApp::new("ui.toml");
    if let Err(err) = evo_ui_engine::runtime::run_app_from_path("ui.toml", app) {
        eprintln!("winit runtime error: {err}");
    }
}
