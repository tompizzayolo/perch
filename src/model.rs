use crate::viewer::ViewerState;
use bevy::{gltf::GltfAssetLabel, prelude::*};
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn load_command_line_model(
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

pub fn handle_file_drop(
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
