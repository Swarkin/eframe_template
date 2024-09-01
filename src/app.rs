use eframe::egui;

#[derive(Default)]
pub struct TemplateApp {}

impl TemplateApp {
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		Default::default()
	}
}

impl eframe::App for TemplateApp {
	fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {

	}
}
