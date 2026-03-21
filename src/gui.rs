use crate::core;
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

pub fn launch_gui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 250.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Wii U Common Key Extractor",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
    .unwrap();
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
            ui.vertical_centered(|ui| {
                ui.heading("Wii U Common Key Extractor");
            });
            ui.separator();

            ui.add_space(10.0);

            // COLLAPSED IF: Combined the button click and the file dialog check
            if ui.button("📁 Select otp.bin").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Binary file", &["bin"])
                    .pick_file()
                {
                    if core::is_valid_otp(&path) {
                        self.selected_file = Some(path);
                        self.error_message = None;
                        self.common_key = None;
                    } else {
                        self.error_message =
                            Some("Invalid file. Must be a 1024-byte .bin file.".to_string());
                        self.selected_file = None;
                    }
                }
            }

            if let Some(path) = &self.selected_file {
                ui.label(format!(
                    "Selected: {}",
                    path.file_name().unwrap().to_string_lossy()
                ));

                if ui.button("🚀 Extract Common Key").clicked() {
                    match core::extract_common_key(path) {
                        Ok(key) => {
                            self.common_key =
                                Some(key.iter().map(|byte| format!("{byte:02X}")).collect());
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Error: {}", e));
                            self.common_key = None;
                        }
                    }
                }
            }

            ui.add_space(10.0);

            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::LIGHT_RED, error);
            }

            if let Some(key) = &self.common_key {
                ui.group(|ui| {
                    ui.label("Common Key:");
                    ui.horizontal(|ui| {
                        let mut key_to_display = key.clone();
                        ui.add(
                            egui::TextEdit::singleline(&mut key_to_display)
                                .font(egui::TextStyle::Monospace)
                                .interactive(false),
                        );

                        if ui.button("📋 Copy").clicked() {
                            ui.output_mut(|o| o.copied_text = key.clone());
                        }
                    });
                });
            }
        });
    }
}
