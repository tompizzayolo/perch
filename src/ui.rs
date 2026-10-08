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
    ui_widgets::{SliderStep, SliderValue, ValueChange, slider_self_update},
};

pub fn ui_root() -> impl SceneList {
    bsn_list! {
        Camera2d
        --
        @lighting_panel()
    }
}

fn lighting_panel() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            width: px(260),
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
                @max: 20000.0,
            }
            SliderValue(5000.0)
            SliderStep(100.0)
            AccessibleLabel("Directional light intensity")
            on(slider_self_update)
            on(
                |change: On<ValueChange<f32>>,
                 mut lighting: ResMut<LightingState>| {
                    lighting.illuminance = change.value;
                }
            )
        ]
    }
}
