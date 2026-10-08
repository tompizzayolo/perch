use crate::{lighting, model, ui, viewer};
use bevy::{
    asset::UnapprovedPathMode,
    dev_tools::infinite_grid::InfiniteGridPlugin,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.35)))
        .init_resource::<viewer::ViewerState>()
        .init_resource::<lighting::LightingState>()
        .insert_resource(UiTheme(create_dark_theme()))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                unapproved_path_mode: UnapprovedPathMode::Deny,
                ..default()
            }),
            InfiniteGridPlugin,
            FeathersPlugins,
        ))
        .add_systems(
            Startup,
            (viewer::setup, lighting::setup, ui::ui_root.spawn()),
        )
        .add_systems(
            Update,
            (
                model::load_command_line_model,
                model::handle_file_drop,
                viewer::orbit_camera,
                lighting::sync_directional_light,
            ),
        )
        .run();
}
