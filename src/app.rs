use egui::{
    include_image, Align, CentralPanel, Color32, Context, Frame, Image, Label, Layout, Margin,
    RichText, Rounding, Sense, SidePanel, Stroke, TextEdit, TopBottomPanel, Ui, Widget
};
use egui_extras::{install_image_loaders, Column, TableBuilder};
use std::collections::HashSet;

use crate::IPAK;

#[derive(Default)]
pub struct App {
    assets: HashSet<Asset>,
    text: String,
}

#[derive(Default, Eq, Hash, PartialEq)]
pub struct Asset {
    selected: bool,
    name: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_image_loaders(&cc.egui_ctx);
        Default::default()
    }

    fn show_search(&mut self, ui: &mut Ui) {
        let search_bar = TextEdit::singleline(&mut self.text)
            .hint_text("Search")
            .desired_width(256.0);

        let frame = Frame::default()
            .inner_margin(Margin::same(4.0))
            .outer_margin(Margin::symmetric(0.0, 8.0))
            .stroke(Stroke::new(1.0, Color32::DARK_GRAY));

        frame.show(ui, |ui| search_bar.show(ui));
    }

    fn show_table(&mut self, ui: &mut Ui) {
        TableBuilder::new(ui)
            // Improve visiblity.
            .striped(true)
            //.resizable(true)
            // ...
            .cell_layout(Layout::left_to_right(Align::Center))
            // Expand table to fit entire space.
            .auto_shrink(false)
            // ...
            .sense(Sense::click())
            // Allocate space then define each column.
            .column(Column::auto().at_least(256.0))
            .column(Column::auto().at_least(128.0))
            .column(Column::auto().at_least(128.0))
            .column(Column::auto().at_least(128.0))
            .header(24.0, |mut header| {
                header.col(|ui| {
                    Label::new(RichText::new("Asset Name").strong())
                        .selectable(false)
                        .ui(ui);
                });
                header.col(|ui| {
                    Label::new(RichText::new("Status").strong())
                        .selectable(false)
                        .ui(ui);
                });
                header.col(|ui| {
                    Label::new(RichText::new("Type").strong())
                        .selectable(false)
                        .ui(ui);
                });
                header.col(|ui| {
                    Label::new(RichText::new("Details").strong())
                        .selectable(false)
                        .ui(ui);
                });
            })
            // TODO: Delete this later.. used for testing.
            .body(|mut body| {
                for asset in &self.assets {
                    body.row(16.0, |mut row| {
                        row.col(|ui| {
                            Label::new(RichText::new(&asset.name).color(Color32::WHITE))
                                .selectable(false)
                                .ui(ui);
                        });
                        row.col(|ui| {
                            Label::new(RichText::new("Unknown").color(Color32::ORANGE))
                                .selectable(false)
                                .ui(ui);
                        });
                        row.col(|ui| {
                            Label::new(RichText::new("Image").color(Color32::WHITE))
                                .selectable(false)
                                .ui(ui);
                        });
                        row.col(|ui| {
                            Label::new(RichText::new("Unknown").color(Color32::WHITE))
                                .selectable(false)
                                .ui(ui);
                        });
                    });
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Header.
        TopBottomPanel::top("head").show(ctx, |ui| {
            self.show_search(ui);
        });

        // Footer.
        TopBottomPanel::bottom("foot").show(ctx, |ui| {
            let frame = Frame::default()
                .inner_margin(Margin::same(2.0))
                .outer_margin(Margin::symmetric(0.0, 8.0))
                .rounding(Rounding::same(4.0))
                .stroke(Stroke::new(1.0, Color32::DARK_GRAY));

            ui.horizontal(|ui| {
                frame.show(ui, |ui| ui.button("Load Game"));
                if frame.show(ui, |ui| ui.button("Load File")).inner.clicked() {
                    let ipak = IPAK::parse("E:/DEV/greenlight/assets/patch.ipak");
                    println!("0x{:08x}", ipak.magic);
                }
                frame.show(ui, |ui| ui.button("Export Selected"));
                frame.show(ui, |ui| ui.button("Export All"));
                frame.show(ui, |ui| ui.button("Clear"));
            });
        });

        // Preview.
        SidePanel::right("preview").show(ctx, |ui| {
            ui.add(Image::new(include_image!(
                "../assets/preview-left-the-oven-on.gif"
            )));
        });

        // Draw after other panels to prevent overlap.
        CentralPanel::default().show(ctx, |ui| {
            let frame = Frame::default()
                .inner_margin(Margin::same(8.0))
                .stroke(Stroke::new(1.0, Color32::DARK_GRAY));

            frame.show(ui, |ui| self.show_table(ui));
        });
    }
}
