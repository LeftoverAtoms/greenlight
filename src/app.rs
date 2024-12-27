use crate::AssetBrowser;

pub struct TemplateApp {
    asset_browser: AssetBrowser,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            asset_browser: Default::default(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            //update_header(ctx, ui);
            self.asset_browser.show(ctx, ui);
        });
    }
}

fn update_header(ctx: &egui::Context, ui:&mut egui::Ui) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        ui.add_space(16.0);

        egui::widgets::global_theme_preference_buttons(ui);
    });
}
