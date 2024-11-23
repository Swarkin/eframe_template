#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
	use eframe::egui;

	env_logger::init();

	eframe::run_native(
		"eframe template",
		eframe::NativeOptions {
			viewport: egui::ViewportBuilder::default()
				.with_inner_size([400.0, 300.0])
				.with_min_inner_size([300.0, 220.0])
				.with_icon(
					eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
						.expect("Failed to load icon"),
				),
			..Default::default()
		},
		Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
	)
}

#[cfg(target_arch = "wasm32")]
fn main() {
	use eframe::wasm_bindgen::JsCast as _;

	eframe::WebLogger::init(log::LevelFilter::Info).ok();

	wasm_bindgen_futures::spawn_local(async {
		let document = web_sys::window()
			.expect("No window")
			.document()
			.expect("No document");

		let canvas = document
			.get_element_by_id("the_canvas_id")
			.expect("Failed to find the_canvas_id")
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.expect("the_canvas_id was not a HtmlCanvasElement");

		let start_result = eframe::WebRunner::new()
			.start(
				canvas,
				eframe::WebOptions::default(),
				Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
			)
			.await;

		if let Some(loading_text) = document.get_element_by_id("loading_text") {
			match start_result {
				Ok(_) => {
					loading_text.remove();
				}
				Err(e) => {
					loading_text.set_inner_html(
						"<p>The app has crashed. See the developer console for details.</p>",
					);
					panic!("Failed to start eframe: {e:?}");
				}
			}
		}
	});
}
