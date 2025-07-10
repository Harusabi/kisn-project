use bevy::ecs::system::ResMut;
use bevy_egui::EguiContexts;
use egui::{CollapsingHeader, CornerRadius, Margin, frame};

use crate::OccupiedScreenSpace;

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    let _frame_top = frame::Frame::new()
        .corner_radius(CornerRadius::same(5))
        .fill(egui::Color32::WHITE)
        .inner_margin(Margin::same(5));

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(ctx, |ui| {
            // ui.set_height(10.0);
            // ctx.set_visuals(egui::Visuals::light());
            let mut styles = (*ctx.style()).clone();

            styles.visuals.window_fill = egui::Color32::WHITE;
            // ctx.set_visuals(egui::Visuals {
            //     window_fill: egui::Color32::GREEN,
            //     ..Default::default()
            // });
            ui.columns(8, |columns| {
                // columns[7].set_style(styles);
                columns[0].label("Column 1");
                CollapsingHeader::new("Collapse")
                    .default_open(false)
                    .show_background(false)
                    .icon(|_, _, _| {})
                    .show(&mut columns[1], |ui| {
                        ui.label("This is the content of column 2.");
                    });

                // if columns[7].button("Quit").clicked() {
                //     std::process::exit(0);
                // }
            });
        })
        .response
        .rect
        .height();
    // occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
    //     .resizable(true)
    //     .show(ctx, |ui| {
    //         ui.label("Bottom resizeable panel");
    //         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    //     })
    //     .response
    //     .rect
    //     .height();
}
