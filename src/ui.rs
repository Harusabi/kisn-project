use bevy::ecs::system::Query;
use bevy_egui::EguiContexts;
use bevy_panorbit_camera::PanOrbitCamera;

pub mod ui_main;

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

pub fn disable_camera_ui(mut context: EguiContexts, mut camera_query: Query<&mut PanOrbitCamera>) {
    // This function is intentionally left empty.
    // It can be used to disable camera UI if needed in the future.
    let ctx = context.ctx_mut();
    let mouse_over_ui = ctx.is_pointer_over_area();

    // Deshabilitar la cámara cuando el mouse está sobre la UI
    for mut camera in camera_query.iter_mut() {
        camera.enabled = !mouse_over_ui;
    }
}
