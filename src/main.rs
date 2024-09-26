#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
	let native_options = eframe::NativeOptions {
		viewport: eframe::egui::ViewportBuilder::default()
			.with_inner_size([400.0, 300.0])
			.with_min_inner_size([300.0, 220.0]),
		..Default::default()
	};
	eframe::run_native(
		"eframe template",
		native_options,
		Box::new(|cc| Ok(Box::new(app::TemplateApp::new(cc)))),
	)
}

#[cfg(target_arch = "wasm32")]
fn main() {
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();

	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		let start_result = eframe::WebRunner::new()
			.start(
				"the_canvas_id",
				web_options,
				Box::new(|cc| Ok(Box::new(app::TemplateApp::new(cc)))),
			)
			.await;

		let loading_text = web_sys::window()
			.and_then(|w| w.document())
			.and_then(|d| d.get_element_by_id("loading_text"));
		if let Some(loading_text) = loading_text {
			match start_result {
				Ok(_) => {
					loading_text.remove();
				}
				Err(e) => {
					loading_text.set_inner_html(
						"<p> The app has crashed. See the developer console for details. </p>",
					);
					panic!("Failed to start eframe: {e:?}");
				}
			}
		}
	});
}
