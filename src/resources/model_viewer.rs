use bevy::prelude::*;
use std::path::PathBuf;

/// Resource to track the current model and animations
#[derive(Resource, Default)]
pub struct ModelViewer {
    pub current_model: Option<Entity>,
    pub model_path: Option<PathBuf>,
    pub gltf_handle: Option<Handle<Gltf>>,
    pub animations: Vec<AnimationNodeIndex>,
    pub animation_names: Vec<String>,
    pub graph_handle: Option<Handle<AnimationGraph>>,
    pub current_animation: usize,
    pub is_playing: bool,
}
