use egui::ImageSource;

pub struct App {
    text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            text: String::default(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        return Default::default();
    }

    fn show_search(&mut self, ui: &mut egui::Ui) {
        egui::TextEdit::singleline(&mut self.text)
            .hint_text("Search")
            .show(ui);
    }

    fn show_table(&mut self, ui: &mut egui::Ui) {
        egui_extras::TableBuilder::new(ui)
            // Improve visiblity.
            .striped(true)
            .resizable(true)
            // ...
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            // Expand table to fit entire space.
            .auto_shrink(false)
            // Allocate space then define each column.
            .column(egui_extras::Column::auto().at_least(128.0))
            .column(egui_extras::Column::auto().at_least(128.0))
            .column(egui_extras::Column::auto().at_least(128.0))
            .column(egui_extras::Column::auto().at_least(128.0))
            .header(24.0, |mut header| {
                header.col(|ui| { ui.strong("Asset Name"); });
                header.col(|ui| { ui.strong("Status"); });
                header.col(|ui| { ui.strong("Type"); });
                header.col(|ui| { ui.strong("Details"); });
            })
            // TODO: Delete this later.. used for testing.
            .body(|mut body| {
                for _ in 0..8 {
                    body.row(16.0, |mut row| {
                        row.col(|ui| { ui.colored_label(egui::Color32::WHITE, "c_zom_player_cia_fb"); });
                        row.col(|ui| { ui.colored_label(egui::Color32::ORANGE, "Placeholder"); });
                        row.col(|ui| { ui.colored_label(egui::Color32::WHITE, "Model"); });
                        row.col(|ui| { ui.colored_label(egui::Color32::WHITE, "Bones: 102, LODs: 4"); });
                    });
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Header.
        egui::TopBottomPanel::top("head")
            .show(ctx, |ui| {
                self.show_search(ui);
            });

        // Footer.
        egui::TopBottomPanel::bottom("foot")
            .show(ctx, |ui| {
                self.show_search(ui);
            });

        // Preview.
        // egui::SidePanel::left("preview")
        //     .show(ctx, |ui| {
        //         ui.add(egui::Image::new(egui::include_image!("../assets/preview-default.png")));
        //     });

        // Draw after other panels to prevent overlap.
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let frame = egui::Frame::default()
                    .inner_margin(egui::Margin::same(16.0))
                    .rounding(egui::Rounding::same(8.0))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::DARK_RED));

                frame.show(ui, |ui| self.show_table(ui));
            });
    }
}
