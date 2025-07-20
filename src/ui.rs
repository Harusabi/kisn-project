use bevy::ecs::system::{Query, ResMut};
use bevy_egui::EguiContexts;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::ui::ui_left::UiState;

pub mod ui_left;
pub mod ui_top;

// fn show_ui_system(world: &mut World) {
//     let Ok(egui_context) = world
//         .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
//         .single(world)
//     else {
//         return;
//     };
//     let mut egui_context = egui_context.clone();

//     world.resource_scope::<UiState, _>(|world, mut ui_state| {
//         ui_state.ui(world, egui_context.get_mut())
//     });
// }

pub fn disable_camera_ui(
    mut context: EguiContexts,
    mut camera_query: Query<&mut PanOrbitCamera>,
    mut rest_uit_state: ResMut<UiState>,
) {
    let ctx = context.ctx_mut();

    if let Some(pointer_position) = ctx.pointer_latest_pos() {
        let mouse_over_ui = ctx.is_pointer_over_area();
        let mouse_gameview = rest_uit_state
            .get_viewport_rect()
            .contains(pointer_position);
        println!(
            "Mouse over UI: {}, Mouse GameView: {}",
            mouse_over_ui, mouse_gameview
        );

        // Deshabilitar la cámara cuando el mouse está sobre la UI
        for mut camera in camera_query.iter_mut() {
            camera.enabled = mouse_gameview || !mouse_over_ui;
        }
    } else {
        for mut camera in camera_query.iter_mut() {
            camera.enabled = false;
        }
    }
}
