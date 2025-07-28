use bevy::ecs::system::ResMut;
use bevy::ecs::system::Single;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::ui::ui_dock::UiState;

pub mod ui_dock;
// pub mod ui_top;

pub fn disable_camera_ui(
    mut camera_query: Single<&mut PanOrbitCamera>,
    mut rest_uit_state: ResMut<UiState>,
) {
    let condicional = rest_uit_state.is_gameview_active();
    camera_query.enabled = condicional;
}
