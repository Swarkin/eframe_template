use eframe::egui;

pub struct TemplateApp {
	label: String,
	value: f32,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			label: "Hello World!".to_owned(),
			value: 2.7,
		}
	}
}

impl TemplateApp {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for TemplateApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						ctx.send_viewport_cmd(egui::ViewportCommand::Close);
					}
				});

				ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
					egui::widgets::global_dark_light_mode_buttons(ui);
				});
			});

		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(&mut self.label);
			});

			ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				self.value += 1.0;
			}

		});
	}
}
