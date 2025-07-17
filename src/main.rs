mod generator;
use crate::generator::generate_reader;

use eframe::egui;
use rfd::FileDialog;

pub struct MyApp {
	folder_path: Option<String>,
	custom_template: Option<String>,
}

impl Default for MyApp {
	fn default() -> Self {
    		Self {
    			folder_path: None,
    			custom_template: None,
    		}
	}
}

impl eframe::App for MyApp {
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
		ui.heading("wcweaver");
	});

	egui::CentralPanel::default().show(ctx, |ui| {
		// File browser for image directory
		ui.heading("Choose directory.");
		if ui.button("Browse").clicked() {
       			if let Some(path) = FileDialog::new().pick_folder() {
       				self.folder_path = Some(path.display().to_string());
			}
		}
		// If a directory was selected, display it
		if let Some(path) = &self.folder_path {
        	        ui.label(format!("Selected folder: {}", path));
		}
		else {
        	        ui.label("No directory selected.");
		}

		// File browser for custom reader template
		ui.heading("Choose HTML template.");
		if ui.button("Browse").clicked() {
            		if let Some(path) = FileDialog::new().add_filter("HTML", &["html"]).pick_file() {
				self.custom_template = Some(path.display().to_string());
			}
		}
           	 if let Some(path) = &self.custom_template {
			ui.label(format!("Selected template: {}", path));
	   	 } 
	   	 else {
	   	 	ui.label("If no custom template is selected, the default template will be used.");
	   	 }

		// "Generate" button
		if let Some(path) = &self.folder_path {
			ui.heading("Generate HTML");
                        if ui.button("Generate").clicked() {
                                generate_reader(&path, &self.custom_template);
                                self.folder_path = None;
                        }

		}

	});
}
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "wcweaver",
        options,
	Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
