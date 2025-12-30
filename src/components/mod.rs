use bevy::prelude::*;

/// Marker for file dialog
pub struct GltfModelFile;

/// Marker for the Open Model button
#[derive(Component)]
pub struct OpenButton;

/// Marker for the Play/Pause button
#[derive(Component)]
pub struct PlayPauseButton;

/// Marker for the animation label text
#[derive(Component)]
pub struct AnimationLabel;

/// Marker for the model name label
#[derive(Component)]
pub struct ModelLabel;

/// Marker for the animation list container
#[derive(Component)]
pub struct AnimationListContainer;

/// Marker for animation list items with index
#[derive(Component)]
pub struct AnimationListItem(pub usize);

/// Marker for "No animations" text
#[derive(Component)]
pub struct NoAnimationsText;

/// Marker for the draggable panel
#[derive(Component)]
pub struct DraggablePanel;

/// Marker for the panel drag area (title bar)
#[derive(Component)]
pub struct PanelDragArea;

/// Marker for the animation scroll area
#[derive(Component)]
pub struct AnimationScrollArea;

/// Marker for entities with animations loaded
#[derive(Component)]
pub struct AnimationsLoaded;
