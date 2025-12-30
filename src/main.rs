use bevy::{
    asset::UnapprovedPathMode,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};
use bevy_file_dialog::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::path::PathBuf;

// Marker for file dialog
struct GltfModelFile;

// Resource to track the current model and animations
#[derive(Resource, Default)]
struct ModelViewer {
    current_model: Option<Entity>,
    model_path: Option<PathBuf>,
    gltf_handle: Option<Handle<Gltf>>,
    animations: Vec<AnimationNodeIndex>,
    animation_names: Vec<String>,
    graph_handle: Option<Handle<AnimationGraph>>,
    current_animation: usize,
    is_playing: bool,
}

// UI component markers
#[derive(Component)]
struct OpenButton;

#[derive(Component)]
struct PlayPauseButton;

#[derive(Component)]
struct AnimationLabel;

#[derive(Component)]
struct ModelLabel;

#[derive(Component)]
struct AnimationListContainer;

#[derive(Component)]
struct AnimationListItem(usize);

#[derive(Component)]
struct NoAnimationsText;

#[derive(Component)]
struct DraggablePanel;

#[derive(Component)]
struct PanelDragArea;

#[derive(Component)]
struct AnimationScrollArea;

// Resource for panel dragging
#[derive(Resource, Default)]
struct PanelDragState {
    dragging: bool,
    offset: Vec2,
}

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

fn setup_scene(mut commands: Commands) {
    // Spawn 3D camera with orbit controls
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::ZERO,
            ..default()
        },
    ));

    // Add ambient light
    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        ..default()
    });

    // Add directional light
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands) {
    // Root UI node (full screen container)
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|parent| {
            // Floating draggable panel - compact design
            parent
                .spawn((
                    DraggablePanel,
                    Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(200.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
                    BorderRadius::all(Val::Px(4.0)),
                ))
                .with_children(|panel| {
                    // Drag handle / Title bar - compact
                    panel
                        .spawn((
                            PanelDragArea,
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                border: UiRect::bottom(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.18, 0.18, 0.18, 1.0)),
                            BorderColor::all(Color::srgba(0.25, 0.25, 0.25, 1.0)),
                            BorderRadius::top(Val::Px(4.0)),
                            RelativeCursorPosition::default(),
                        ))
                        .with_child((
                            Text::new("Model Viewer"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));

                    // Content area - compact padding
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            row_gap: Val::Px(6.0),
                            ..default()
                        })
                        .with_children(|content| {
                            // Open Model button - compact
                            content
                                .spawn((
                                    Button,
                                    OpenButton,
                                    Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.25, 0.25, 0.55)),
                                    BorderRadius::all(Val::Px(3.0)),
                                ))
                                .with_child((
                                    Text::new("Open Model..."),
                                    TextFont {
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));

                            // Model name label - smaller, with wrapping
                            content.spawn((
                                ModelLabel,
                                Node {
                                    width: Val::Percent(100.0),
                                    ..default()
                                },
                                Text::new("No model loaded"),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                TextLayout::new_with_linebreak(LineBreak::WordBoundary),
                            ));

                            // Separator
                            content.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(1.0),
                                    margin: UiRect::axes(Val::Px(0.0), Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                            ));

                            // Current animation label (no separate title)
                            content.spawn((
                                Text::new("Selected: None"),
                                AnimationLabel,
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));

                            // Animation list (scrollable)
                            content
                                .spawn((
                                    AnimationScrollArea,
                                    Interaction::default(),
                                    RelativeCursorPosition::default(),
                                    ScrollPosition::default(),
                                    Node {
                                        width: Val::Percent(100.0),
                                        max_height: Val::Px(180.0),
                                        flex_direction: FlexDirection::Column,
                                        overflow: Overflow::scroll_y(),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.08, 0.08, 0.08, 0.9)),
                                    BorderRadius::all(Val::Px(3.0)),
                                ))
                                .with_children(|list_container| {
                                    list_container.spawn((
                                        AnimationListContainer,
                                        Node {
                                            flex_direction: FlexDirection::Column,
                                            width: Val::Percent(100.0),
                                            ..default()
                                        },
                                    ));
                                });

                            // Play/Pause button - compact
                            content
                                .spawn((
                                    Button,
                                    PlayPauseButton,
                                    Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
                                        justify_content: JustifyContent::Center,
                                        margin: UiRect::top(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.45, 0.2)),
                                    BorderRadius::all(Val::Px(3.0)),
                                ))
                                .with_child((
                                    Text::new("Play"),
                                    TextFont {
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                        });
                });
        });
}

fn drag_panel(
    mut panel_query: Query<&mut Node, With<DraggablePanel>>,
    drag_area_query: Query<(&Interaction, &RelativeCursorPosition), With<PanelDragArea>>,
    mut drag_state: ResMut<PanelDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    for (interaction, relative_pos) in &drag_area_query {
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

fn button_interactions(
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
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.7));
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
                    *bg = BackgroundColor(Color::srgb(0.2, 0.5, 0.2));
                }
            }
        }
    }
}

fn animation_list_interactions(
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
                    *bg = BackgroundColor(Color::srgb(0.2, 0.4, 0.6));
                } else {
                    *bg = BackgroundColor(Color::NONE);
                }
            }
        }
    }
}

fn update_animation_list(
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

fn update_ui_labels(
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
                    *bg = BackgroundColor(Color::srgb(0.2, 0.5, 0.2));
                }
            }
        }
    }
}

fn handle_loaded_model(
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

#[derive(Component)]
struct AnimationsLoaded;

fn setup_animations(
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

fn control_animations(
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

fn disable_camera_on_ui_hover(
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

fn scroll_animation_list(
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
