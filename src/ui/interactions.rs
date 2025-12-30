use bevy::prelude::*;
use bevy_file_dialog::prelude::*;

use crate::components::*;
use crate::resources::ModelViewer;

pub fn button_interactions(
    mut commands: Commands,
    mut viewer: ResMut<ModelViewer>,
    mut open_btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OpenButton>),
    >,
    mut play_btn: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<PlayPauseButton>,
            Without<OpenButton>,
        ),
    >,
) {
    // Open button
    for (interaction, mut bg) in &mut open_btn {
        match *interaction {
            Interaction::Pressed => {
                commands
                    .dialog()
                    .add_filter("glTF", &["gltf", "glb"])
                    .load_file::<GltfModelFile>();
                *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.5));
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.55));
            }
        }
    }

    // Play/Pause button
    for (interaction, mut bg) in &mut play_btn {
        match *interaction {
            Interaction::Pressed => {
                viewer.is_playing = !viewer.is_playing;
            }
            Interaction::Hovered => {
                if viewer.is_playing {
                    *bg = BackgroundColor(Color::srgb(0.7, 0.3, 0.3));
                } else {
                    *bg = BackgroundColor(Color::srgb(0.3, 0.6, 0.3));
                }
            }
            Interaction::None => {
                if viewer.is_playing {
                    *bg = BackgroundColor(Color::srgb(0.5, 0.2, 0.2));
                } else {
                    *bg = BackgroundColor(Color::srgb(0.2, 0.45, 0.2));
                }
            }
        }
    }
}

pub fn animation_list_interactions(
    mut viewer: ResMut<ModelViewer>,
    mut items: Query<
        (&Interaction, &AnimationListItem, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, item, mut bg) in &mut items {
        let is_selected = viewer.current_animation == item.0;
        match *interaction {
            Interaction::Pressed => {
                viewer.current_animation = item.0;
                viewer.is_playing = true;
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.5));
            }
            Interaction::None => {
                if is_selected {
                    *bg = BackgroundColor(Color::srgb(0.2, 0.35, 0.5));
                } else {
                    *bg = BackgroundColor(Color::NONE);
                }
            }
        }
    }
}
