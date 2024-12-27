use egui::{Align, Layout, Ui};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct AssetBrowser { }

impl AssetBrowser {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // Expand as much as possible.
        let height: f32 = ctx.available_rect().height();
        ui.set_max_height(height);
        // ...
        ui.vertical_centered_justified(|ui| {
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
        });
    }
}
