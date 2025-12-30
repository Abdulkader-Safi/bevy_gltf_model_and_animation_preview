mod components;
mod resources;
mod systems;
mod ui;

use bevy::{asset::UnapprovedPathMode, prelude::*};
use bevy_file_dialog::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use components::GltfModelFile;
use resources::{ModelViewer, PanelDragState};
use systems::{
    control_animations, disable_camera_on_ui_hover, drag_panel, handle_loaded_model,
    scroll_animation_list, setup_animations,
};
use ui::{
    animation_list_interactions, button_interactions, setup_scene, setup_ui, update_animation_list,
    update_ui_labels,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "GLTF Model & Animation Preview".to_string(),
                        resizable: true,
                        resolution: (1280, 720).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                }),
        )
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(FileDialogPlugin::new().with_load_file::<GltfModelFile>())
        .init_resource::<ModelViewer>()
        .init_resource::<PanelDragState>()
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(
            Update,
            (
                handle_loaded_model,
                setup_animations,
                control_animations,
                button_interactions,
                animation_list_interactions,
                update_ui_labels,
                update_animation_list,
                drag_panel,
                disable_camera_on_ui_hover,
                scroll_animation_list,
            ),
        )
        .run();
}
