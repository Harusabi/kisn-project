mod ui;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{EguiContextPass, EguiPlugin};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use ui::disable_camera_ui;
use ui::ui_left::show_ui_system;
use ui::ui_top::ui_top_panel;

use crate::ui::ui_left::UiState;

#[derive(Default, Resource)]
// #[allow(dead_code)]
struct OccupiedScreenSpace {
    _left: f32,
    top: f32,
    _right: f32,
    _bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin,
            InfiniteGridPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .insert_resource(UiState::new())
        .init_resource::<OccupiedScreenSpace>()
        .add_systems(Startup, setup_system)
        // .add_systems(Update, disable_camera_ui)
        .add_systems(EguiContextPass, (show_ui_system))
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGridBundle::default());

    commands.spawn((
        Name::new("Suelo"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    commands.spawn((
        Name::new("Cubo"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let camera_pos = Vec3::new(-10.0, 10.0, 30.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((
        Transform::from_xyz(3.0, 6.0, 0.0),
        PanOrbitCamera::default(),
    ));
}
