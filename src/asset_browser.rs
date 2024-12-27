use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct AssetBrowser { }

impl AssetBrowser {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(800.0);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Clipped text");
                    });
                    header.col(|ui| {
                        ui.strong("Expanding content");
                    });
                    header.col(|ui| {
                        ui.strong("Interaction");
                    });
                    header.col(|ui| {
                        ui.strong("Content");
                    });
                })
                .body(|mut body| {
                    body.row(12.0, |mut row| {
                        
                    }
                );
            });
        });
    }
}
