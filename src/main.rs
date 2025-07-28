use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{EguiContextPass, EguiPlugin};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod ui;
use ui::disable_camera_ui;
use ui::ui_dock::UiState;
use ui::ui_dock::show_ui_system;

#[derive(Default, Resource)]
#[allow(dead_code)]
struct OccupiedScreenSpace {
    _left: f32,
    top: f32,
    _right: f32,
    _bottom: f32,
}

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kisn Project".to_string(),
                resolution: (1024.0, 768.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
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
        .add_systems(Update, disable_camera_ui)
        .add_systems(EguiContextPass, show_ui_system)
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

    let bundle = Name::new("Cubo");
    commands.spawn(bundle).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
    });

    commands.spawn((
        PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Transform::from_xyz(3.0, 6.0, 0.0),
        PanOrbitCamera::default(),
    ));
}
