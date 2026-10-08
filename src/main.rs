use bevy::{
    asset::UnapprovedPathMode,
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings},
    gltf::GltfAssetLabel,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Resource, Default)]
struct ViewerState {
    model: Option<Entity>,
}

#[derive(Component)]
struct ViewerCamera {
    base_target: Vec3,
    offset: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.025, 0.03)))
        .init_resource::<ViewerState>()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                unapproved_path_mode: UnapprovedPathMode::Deny,
                ..default()
            }),
            InfiniteGridPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (load_command_line_model, handle_file_drop, orbit_camera),
        )
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.35)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        // You need to spawn an entity with this component
        InfiniteGrid,
        // Optional component you can use to configure the grid
        InfiniteGridSettings::default(),
    ));

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

    commands.spawn((AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    },));
}

fn load_command_line_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut viewer_state: ResMut<ViewerState>,
) {
    let Some(path) = env::args_os().nth(1).map(PathBuf::from) else {
        return;
    };

    if viewer_state.model.is_none() {
        load_model(&mut commands, &asset_server, &mut viewer_state, path);
    }
}

fn handle_file_drop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut viewer_state: ResMut<ViewerState>,
    mut events: MessageReader<FileDragAndDrop>,
) {
    for event in events.read() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };

        if is_gltf_file(path_buf) {
            load_model(
                &mut commands,
                &asset_server,
                &mut viewer_state,
                path_buf.clone(),
            );
        }
    }
}

fn load_model(
    commands: &mut Commands,
    asset_server: &AssetServer,
    viewer_state: &mut ViewerState,
    path: PathBuf,
) {
    if !is_gltf_file(&path) {
        return;
    }

    if let Some(previous_model) = viewer_state.model.take() {
        commands.entity(previous_model).despawn();
    }

    let path = path.canonicalize().unwrap_or(path);

    let scene = asset_server
        .load_builder()
        .override_unapproved()
        .load(GltfAssetLabel::Scene(0).from_asset(path));

    let entity = commands
        .spawn((WorldAssetRoot(scene), Transform::default()))
        .id();

    viewer_state.model = Some(entity);
}

fn is_gltf_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("glb" | "gltf")
    )
}

fn orbit_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut ViewerCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };

    let control = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
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
