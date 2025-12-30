use bevy::prelude::*;

/// Resource for tracking panel dragging state
#[derive(Resource, Default)]
pub struct PanelDragState {
    pub dragging: bool,
    pub offset: Vec2,
}
