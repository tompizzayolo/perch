use crate::lighting::LightingState;
use bevy::{
    feathers::{
        controls::*,
        display::{caption, label},
        theme::ThemeBackgroundColor,
        tokens,
    },
    prelude::*,
    ui::Checked,
    ui_widgets::{SliderValue, ValueChange, slider_self_update},
};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

pub fn ui_root() -> impl SceneList {
    bsn_list! {
        @ui_container()
    }
}

fn ui_container() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            padding: px(12),
        }
        Children [
            @lighting_panel()
        ]
    }
}

fn lighting_panel() -> impl Scene {
    bsn! {
        Node {
            width: px(280),
            min_width: px(280),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            padding: px(12),
        }
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            @label("Lighting")
            --
            @FeathersCheckbox {
                @caption: bsn! {
                    @caption("Directional light")
                }
            }
            Checked
            AccessibleLabel("Directional light")
            on(
                |change: On<ValueChange<bool>>,
                 mut lighting: ResMut<LightingState>,
                 mut commands: Commands| {
                    lighting.enabled = change.value;

                    let mut checkbox = commands.entity(change.source);
                    if change.value {
                        checkbox.insert(Checked);
                    } else {
                        checkbox.remove::<Checked>();
                    }
                }
            )
            --
            @label("Intensity")
            --
            @FeathersSlider {
                @max: 100.0,
            }
            SliderValue(25.0)
            Node {
                width: percent(100),
                min_width: px(220),
                height: px(24),
            }
            AccessibleLabel("Directional light intensity")
            on(slider_self_update)
            on(
                |change: On<ValueChange<f32>>,
                 mut lighting: ResMut<LightingState>| {
                    lighting.illuminance = change.value * 200.0;
                }
            )
        ]
    }
}
