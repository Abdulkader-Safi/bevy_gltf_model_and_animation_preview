use bevy::prelude::*;

use crate::components::*;
use crate::resources::ModelViewer;

pub fn update_animation_list(
    mut commands: Commands,
    viewer: Res<ModelViewer>,
    list_container: Query<Entity, With<AnimationListContainer>>,
    existing_items: Query<Entity, With<AnimationListItem>>,
    no_anim_text: Query<Entity, With<NoAnimationsText>>,
) {
    if !viewer.is_changed() {
        return;
    }

    let Ok(container) = list_container.single() else {
        return;
    };

    // Clear existing items
    for entity in &existing_items {
        commands.entity(entity).despawn();
    }
    for entity in &no_anim_text {
        commands.entity(entity).despawn();
    }

    // Add new items
    commands.entity(container).with_children(|parent| {
        if viewer.animation_names.is_empty() {
            parent.spawn((
                NoAnimationsText,
                Text::new("No animations"),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.4, 0.4)),
                Node {
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
            ));
        } else {
            for (i, name) in viewer.animation_names.iter().enumerate() {
                let is_selected = i == viewer.current_animation;
                parent
                    .spawn((
                        Button,
                        AnimationListItem(i),
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                            ..default()
                        },
                        if is_selected {
                            BackgroundColor(Color::srgb(0.2, 0.35, 0.5))
                        } else {
                            BackgroundColor(Color::NONE)
                        },
                    ))
                    .with_child((
                        Text::new(name.clone()),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(if is_selected {
                            Color::WHITE
                        } else {
                            Color::srgb(0.75, 0.75, 0.75)
                        }),
                    ));
            }
        }
    });
}

pub fn update_ui_labels(
    viewer: Res<ModelViewer>,
    mut model_label: Query<&mut Text, (With<ModelLabel>, Without<AnimationLabel>)>,
    mut anim_label: Query<&mut Text, (With<AnimationLabel>, Without<ModelLabel>)>,
    mut play_btn: Query<(&Children, &mut BackgroundColor), With<PlayPauseButton>>,
    mut play_btn_text: Query<&mut Text, (Without<ModelLabel>, Without<AnimationLabel>)>,
) {
    if !viewer.is_changed() {
        return;
    }

    // Update model label
    for mut text in &mut model_label {
        if let Some(path) = &viewer.model_path {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            **text = format!("Model: {}", name);
        } else {
            **text = "No model loaded".to_string();
        }
    }

    // Update animation label
    for mut text in &mut anim_label {
        if viewer.animation_names.is_empty() {
            **text = "Selected: None".to_string();
        } else if let Some(name) = viewer.animation_names.get(viewer.current_animation) {
            **text = format!("Selected: {}", name);
        }
    }

    // Update play/pause button
    for (children, mut bg) in &mut play_btn {
        for child in children.iter() {
            if let Ok(mut text) = play_btn_text.get_mut(child) {
                if viewer.is_playing {
                    **text = "Pause".to_string();
                    *bg = BackgroundColor(Color::srgb(0.5, 0.2, 0.2));
                } else {
                    **text = "Play".to_string();
                    *bg = BackgroundColor(Color::srgb(0.2, 0.45, 0.2));
                }
            }
        }
    }
}
