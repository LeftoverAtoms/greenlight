use egui::{Align, Layout, Ui};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct AssetBrowser {
    pub text: String,
}

impl AssetBrowser {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        self.show_search(ui);

        ui.vertical_centered_justified(|ui| {
            self.show_table(ui);
        });
    }

    fn show_search(&mut self, ui: &mut Ui) {
        egui::TextEdit::singleline(&mut self.text)
            .hint_text("Search")
            .show(ui);
    }

    fn show_table(&mut self, ui: &mut Ui) {
        TableBuilder::new(ui)
            // Style.
            .cell_layout(Layout::left_to_right(Align::Center))
            .striped(true)
            // Must allocate space foreach column.
            .column(Column::remainder())
            // ...
            .header(20.0, |mut header| {
                // Define each column.
                header.col(|ui| {
                    ui.strong("Interaction");
                });
            })
            .body(|mut body| {
                for row_index in 0..8 {
                    body.row(64.0, |mut row| {
                        row.col(|ui| {
                            ui.checkbox(&mut true, "Click me");
                        });
                    });
                }
            });
    }
}
