use bevy::prelude::*;

use crate::components::AnimationsLoaded;
use crate::resources::ModelViewer;

pub fn setup_animations(
    mut commands: Commands,
    mut viewer: ResMut<ModelViewer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    gltf_assets: Res<Assets<Gltf>>,
    children_query: Query<&Children>,
    animation_player_query: Query<Entity, With<AnimationPlayer>>,
    animations_loaded_query: Query<(), With<AnimationsLoaded>>,
) {
    // Only proceed if we have a model and no animations loaded yet
    let Some(model_entity) = viewer.current_model else {
        return;
    };

    // Already loaded animations
    if !viewer.animations.is_empty() || !animations_loaded_query.is_empty() {
        return;
    }

    // Use the stored GLTF handle
    let Some(gltf_handle) = &viewer.gltf_handle else {
        return;
    };

    let Some(gltf) = gltf_assets.get(gltf_handle) else {
        // GLTF not loaded yet
        return;
    };

    info!("GLTF asset loaded!");
    info!("  - animations: {}", gltf.animations.len());
    info!("  - named_animations: {}", gltf.named_animations.len());
    info!("  - scenes: {}", gltf.scenes.len());
    info!("  - named_scenes: {}", gltf.named_scenes.len());

    // Find animation player in hierarchy
    let mut player_entity = None;
    let mut to_check = vec![model_entity];
    while let Some(entity) = to_check.pop() {
        if animation_player_query.get(entity).is_ok() {
            player_entity = Some(entity);
            break;
        }
        if let Ok(children) = children_query.get(entity) {
            to_check.extend(children.iter());
        }
    }

    let Some(player_entity) = player_entity else {
        // AnimationPlayer not found yet - scene might still be loading
        return;
    };

    if gltf.animations.is_empty() {
        // Mark as loaded even if no animations
        commands.entity(player_entity).insert(AnimationsLoaded);
        info!("No animations found in GLTF");
        return;
    }

    info!("Found {} animations in GLTF", gltf.animations.len());

    // Create animation graph and get animation names
    let (graph, indices) = AnimationGraph::from_clips(gltf.animations.iter().cloned());
    let graph_handle = graphs.add(graph);

    // Get animation names from the named_animations map
    let mut animation_names: Vec<String> = Vec::new();

    // Use named_animations directly - it maps name to handle
    for (name, _handle) in gltf.named_animations.iter() {
        animation_names.push(name.to_string());
    }

    // Sort alphabetically for consistent ordering
    animation_names.sort();

    // If no named animations, create default names
    if animation_names.is_empty() {
        for i in 0..gltf.animations.len() {
            animation_names.push(format!("Animation {}", i + 1));
        }
    }

    info!("Animation names: {:?}", animation_names);

    viewer.animations = indices;
    viewer.animation_names = animation_names;
    viewer.graph_handle = Some(graph_handle.clone());
    viewer.is_playing = true;

    // Add the graph to the animation player
    commands
        .entity(player_entity)
        .insert((AnimationGraphHandle(graph_handle), AnimationsLoaded));
}

pub fn control_animations(
    viewer: Res<ModelViewer>,
    mut animation_players: Query<(&mut AnimationPlayer, &AnimationGraphHandle)>,
) {
    if viewer.animations.is_empty() {
        return;
    };

    for (mut player, _graph) in &mut animation_players {
        let animation_index = viewer.animations[viewer.current_animation];

        if viewer.is_playing {
            if !player.is_playing_animation(animation_index) {
                // Stop all other animations and play the selected one
                player.stop_all();
                player.play(animation_index).repeat();
            }
            player.resume_all();
        } else {
            player.pause_all();
        }
    }
}
