use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::CameraController;

pub fn move_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &CameraController)>,
    mut contexts: EguiContexts,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    // Verificar si el mouse está sobre la UI de egui
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    // Solo mover cámara si se mantiene presionado el botón derecho
    if !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if delta.length_squared() > 0.0 {
        for (mut transform, controller) in query.iter_mut() {
            if controller.enabled {
                // Rotación horizontal (yaw)
                transform.rotate_y(-delta.x * controller.sensitivity);

                // Rotación vertical (pitch) con límites
                let pitch_delta = -delta.y * controller.sensitivity;
                let right = transform.rotation * Vec3::X;

                // Limitar el pitch
                let current_pitch = transform.rotation.to_euler(EulerRot::YXZ).1;
                let new_pitch = (current_pitch + pitch_delta).clamp(-1.5, 1.5);
                let pitch_diff = new_pitch - current_pitch;

                if pitch_diff.abs() > 0.001 {
                    let transfor_translation = transform.translation;
                    transform.rotate_around(
                        transfor_translation,
                        Quat::from_axis_angle(right, pitch_diff),
                    );
                }
            }
        }
    }
}
