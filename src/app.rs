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
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		Default::default()
	}
}

impl eframe::App for TemplateApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				if !cfg!(target_arch = "wasm32") {
					ui.menu_button("File", |ui| {
						if ui.button("Quit").clicked() {
							ctx.send_viewport_cmd(egui::ViewportCommand::Close);
						}
					});
					ui.add_space(16.0);
				}

				egui::widgets::global_dark_light_mode_buttons(ui);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("eframe template");

			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(&mut self.label);
			});

			ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				self.value += 1.0;
			}

			ui.separator();

			ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				powered_by_egui_and_eframe(ui);
				egui::warn_if_debug_build(ui);
			});
		});
	}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.spacing_mut().item_spacing.x = 0.0;
		ui.label("Powered by ");
		ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		ui.label(" and ");
		ui.hyperlink_to(
			"eframe",
			"https://github.com/emilk/egui/tree/master/crates/eframe",
		);
		ui.label(".");
	});
}
