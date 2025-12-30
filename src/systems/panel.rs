use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crate::components::{AnimationScrollArea, DraggablePanel, PanelDragArea};
use crate::resources::PanelDragState;

pub fn drag_panel(
    mut panel_query: Query<&mut Node, With<DraggablePanel>>,
    drag_area_query: Query<(&Interaction, &RelativeCursorPosition), With<PanelDragArea>>,
    mut drag_state: ResMut<PanelDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    for (interaction, _relative_pos) in &drag_area_query {
        if *interaction == Interaction::Pressed && !drag_state.dragging {
            drag_state.dragging = true;
            if let Ok(panel) = panel_query.single() {
                let panel_x = match panel.left {
                    Val::Px(x) => x,
                    _ => 0.0,
                };
                let panel_y = match panel.top {
                    Val::Px(y) => y,
                    _ => 0.0,
                };
                drag_state.offset = Vec2::new(cursor_pos.x - panel_x, cursor_pos.y - panel_y);
            }
        }
    }

    if !mouse_button.pressed(MouseButton::Left) {
        drag_state.dragging = false;
    }

    if drag_state.dragging {
        if let Ok(mut panel) = panel_query.single_mut() {
            panel.left = Val::Px((cursor_pos.x - drag_state.offset.x).max(0.0));
            panel.top = Val::Px((cursor_pos.y - drag_state.offset.y).max(0.0));
        }
    }
}

pub fn scroll_animation_list(
    mut scroll_query: Query<
        (&RelativeCursorPosition, &mut ScrollPosition),
        With<AnimationScrollArea>,
    >,
    mut mouse_wheel: MessageReader<MouseWheel>,
) {
    for (rel_pos, mut scroll_pos) in &mut scroll_query {
        if !rel_pos.cursor_over() {
            continue;
        }

        for event in mouse_wheel.read() {
            let dy = match event.unit {
                MouseScrollUnit::Line => event.y * 20.0,
                MouseScrollUnit::Pixel => event.y,
            };
            scroll_pos.y = (scroll_pos.y - dy).max(0.0);
        }
    }
}
