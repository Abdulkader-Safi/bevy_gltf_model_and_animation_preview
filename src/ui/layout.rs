use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::*;

pub fn setup_scene(mut commands: Commands) {
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

pub fn setup_ui(mut commands: Commands) {
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
