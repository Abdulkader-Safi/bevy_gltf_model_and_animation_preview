use bevy::prelude::*;
use bevy_file_dialog::prelude::*;

use crate::components::GltfModelFile;
use crate::resources::ModelViewer;

pub fn handle_loaded_model(
    mut ev: MessageReader<DialogFileLoaded<GltfModelFile>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut viewer: ResMut<ModelViewer>,
) {
    for event in ev.read() {
        // Despawn previous model
        if let Some(entity) = viewer.current_model {
            commands.entity(entity).despawn();
        }

        // Reset viewer state
        viewer.animations.clear();
        viewer.animation_names.clear();
        viewer.graph_handle = None;
        viewer.gltf_handle = None;
        viewer.current_animation = 0;
        viewer.is_playing = false;

        // Load new model
        let path = event.path.clone();
        viewer.model_path = Some(path.clone());

        // Load the GLTF asset and store the handle for animation loading
        let gltf_handle: Handle<Gltf> = asset_server.load(path.clone());
        viewer.gltf_handle = Some(gltf_handle.clone());

        // Load the scene from the GLTF
        let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()));
        let entity = commands
            .spawn((SceneRoot(scene), Transform::default()))
            .id();

        viewer.current_model = Some(entity);
    }
}
