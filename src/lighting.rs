use bevy::prelude::*;

#[derive(Resource)]
pub struct LightingState {
    pub enabled: bool,
    pub illuminance: f32,
}

impl Default for LightingState {
    fn default() -> Self {
        Self {
            enabled: true,
            illuminance: 5000.0,
        }
    }
}

#[derive(Component)]
pub struct ViewerDirectionalLight;

pub fn setup(mut commands: Commands, lighting: Res<LightingState>) {
    commands.spawn((AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    },));

    let illuminance = if lighting.enabled {
        lighting.illuminance
    } else {
        0.0
    };

    commands.spawn((
        DirectionalLight {
            illuminance,
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ViewerDirectionalLight,
    ));
}

pub fn sync_directional_light(
    lighting: Res<LightingState>,
    mut lights: Query<&mut DirectionalLight, With<ViewerDirectionalLight>>,
) {
    if !lighting.is_changed() {
        return;
    }

    if !lighting.illuminance.is_finite() {
        return;
    }

    let illuminance = if lighting.enabled {
        lighting.illuminance.clamp(0.0, 20000.0)
    } else {
        0.0
    };

    for mut light in lights.iter_mut() {
        light.illuminance = illuminance;
    }
}
