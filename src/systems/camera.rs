use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::{AnimationScrollArea, DraggablePanel};

pub fn disable_camera_on_ui_hover(
    panel_query: Query<&Interaction, With<DraggablePanel>>,
    scroll_query: Query<&RelativeCursorPosition, With<AnimationScrollArea>>,
    mut camera_query: Query<&mut PanOrbitCamera>,
) {
    let mut over_ui = false;

    // Check if hovering over the panel
    for interaction in &panel_query {
        if *interaction != Interaction::None {
            over_ui = true;
            break;
        }
    }

    // Check if cursor is inside scroll area
    for rel_pos in &scroll_query {
        if rel_pos.cursor_over() {
            over_ui = true;
            break;
        }
    }

    // Disable/enable camera based on UI hover
    for mut camera in &mut camera_query {
        camera.enabled = !over_ui;
    }
}
