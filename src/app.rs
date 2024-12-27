use crate::AssetBrowser;
use crate::Header;

pub struct App {
    asset_browser: AssetBrowser,
    header: Header,
}

impl Default for App {
    fn default() -> Self {
        Self {
            asset_browser: Default::default(),
            header: Default::default(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.header.show(ctx, ui);
            ui.add_space(8.0);
            self.asset_browser.show_search(ui);
            ui.add_space(8.0);
            self.asset_browser.show_table(ui);
        });

        self.header.update(ctx);
    }
}
