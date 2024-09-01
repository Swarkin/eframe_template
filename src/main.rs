mod app;

fn main() -> eframe::Result {
    eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::TemplateApp::new(cc)))),
    )
}
