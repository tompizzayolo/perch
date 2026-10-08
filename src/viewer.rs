use bevy::{
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridSettings},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Resource, Default)]
pub struct ViewerState {
    pub model: Option<Entity>,
}

#[derive(Component)]
pub struct ViewerCamera {
    pub base_target: Vec3,
    pub offset: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

pub fn setup(mut commands: Commands) {
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ViewerCamera {
            base_target: Vec3::ZERO,
            offset: Vec3::ZERO,
            distance: 5.0,
            yaw: 0.0,
            pitch: -0.15,
        },
    ));
}

pub fn orbit_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut ViewerCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };

    let control = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ControlRight);
    let middle = mouse_buttons.pressed(MouseButton::Middle);

    let mut consumed_mouse_motion = false;

    if middle {
        for motion in mouse_motion.read() {
            consumed_mouse_motion = true;

            let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);

            if control {
                let right = rotation * Vec3::X;
                let up = rotation * Vec3::Y;
                let scale = camera.distance * 0.002;

                camera.offset -= right * (motion.delta.x * scale);
                camera.offset += up * (motion.delta.y * scale);
            } else {
                camera.yaw -= motion.delta.x * 0.008;
                camera.pitch -= motion.delta.y * 0.008;
                camera.pitch = camera.pitch.clamp(-1.5, 1.5);
            }
        }
    }

    if !consumed_mouse_motion {
        mouse_motion.clear();
    }

    for wheel in mouse_wheel.read() {
        camera.distance *= 1.0 - wheel.y * 0.1;
        camera.distance = camera.distance.clamp(0.05, 1000.0);
    }

    if keyboard.just_pressed(KeyCode::Period) {
        camera.offset = Vec3::ZERO;
    }

    let focus = camera.base_target + camera.offset;
    let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);

    transform.translation = focus + rotation * Vec3::new(0.0, 0.0, camera.distance);
    transform.look_at(focus, Vec3::Y);
}
