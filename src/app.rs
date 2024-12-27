use crate::AssetBrowser;

pub struct App {
    asset_browser: AssetBrowser,
}

impl Default for App {
    fn default() -> Self {
        Self {
            asset_browser: Default::default(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn update_header(&mut self, ctx: &egui::Context, ui:&mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load File").clicked() {
                    ui.close_menu();
                }
    
                if ui.button("Export Selected").clicked() {
                    ui.close_menu();
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.update_header(ctx, ui);
            ui.add_space(8.0);
            self.asset_browser.show_search(ui);
            ui.add_space(8.0);
            self.asset_browser.show_table(ui);
        });
    }
}
