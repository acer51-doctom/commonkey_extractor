use crate::core;
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

pub fn launch_gui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Common Key Extractor",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).unwrap();
}

#[derive(Default)]
struct MyApp {
    selected_file: Option<PathBuf>,
    common_key: Option<String>,
    error_message: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wii U Common Key Extractor");
            ui.separator();

            if ui.button("Select otp.bin").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Binary file", &["bin"])
                    .pick_file()
                {
                    self.selected_file = Some(path);
                    self.error_message = None;
                    self.common_key = None;
                }
            }

            if let Some(path) = &self.selected_file {
                ui.horizontal(|ui| {
                    ui.label("Selected file:");
                    ui.monospace(path.display().to_string());
                });

                if !core::is_valid_otp(path) {
                    self.error_message = Some("This is not a valid otp.bin file. It must be a .bin file of 1024 bytes.".to_string());
                    self.selected_file = None;
                }
            }
            
            ui.separator();

            if self.selected_file.is_some() {
                 if ui.button("Extract Common Key").clicked() {
                    if let Some(path) = &self.selected_file {
                        match core::extract_common_key(path) {
                            Ok(key) => {
                                self.common_key = Some(key.iter().map(|byte| format!("{byte:02X}")).collect());
                                self.error_message = None;
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Error extracting key: {}", e));
                                self.common_key = None;
                            }
                        }
                    }
                }
            }

            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(key) = &self.common_key {
                ui.horizontal(|ui| {
                    ui.label("Common Key:");
                    ui.text_edit_singleline(&mut key.clone());
                });
            }
        });
    }
}
