use egui::{Align, Layout, Ui};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct AssetBrowser {
    pub text: String,
}

impl AssetBrowser {
    pub fn show_search(&mut self, ui: &mut Ui) {
        egui::TextEdit::singleline(&mut self.text)
            .hint_text("Search")
            .show(ui);
    }

    pub fn show_table(&mut self, ui: &mut Ui) {
        TableBuilder::new(ui)
            // Style.
            .cell_layout(Layout::left_to_right(Align::Center))
            .striped(true)
            .resizable(true)
            // Must allocate space foreach column.
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            // ...
            .header(20.0, |mut header| {
                // Define each column.
                header.col(|ui| {
                    ui.strong("Asset Name");
                });
                header.col(|ui| {
                    ui.strong("Type");
                });
                header.col(|ui| {
                    ui.strong("Details");
                });
            })
            .body(|mut body| {
                for row_index in 0..8 {
                    body.row(64.0, |mut row| {
                        row.col(|ui| {
                            ui.label("c_zom_player_cia_fb");
                        });
                        row.col(|ui| {
                            ui.label("Model");
                        });
                        row.col(|ui| {
                            ui.label("Bones: 102, LODs: 4");
                        });
                    });
                }
            });
    }
}
