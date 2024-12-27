use egui::Ui;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct Header {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
        }
    }
}

impl Header {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load File").clicked() {
                    ui.close_menu();
                    self.file_dialog.pick_file();
                }
    
                if ui.button("Export Selected").clicked() {
                    ui.close_menu();
                    self.file_dialog.save_file();
                }
    
                if ui.button("Quit").clicked() {
                    ui.close_menu();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.add_space(16.0);

            egui::widgets::global_theme_preference_buttons(ui);
        });
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked() {
            self.picked_file = Some(path.to_path_buf());
            println!("{}", path.to_path_buf().display());
        }
    }
}