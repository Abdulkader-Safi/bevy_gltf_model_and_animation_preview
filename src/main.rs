use bevy::prelude::*;
use bevy_egui::egui::Shadow;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GLTF Model & Animation Preview".to_string(),
                resizable: true,
                resolution: (1280, 720).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        .register_type::<Configuration>() // you need to register your type to display it
        .add_plugins(EguiPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they are `Reflect`
        // .add_plugins(ResourceInspectorPlugin::<Time>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, configure_visuals.run_if(run_once))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn configure_visuals(mut contexts: EguiContexts) {
    if let Ok(ctx) = contexts.ctx_mut() {
        ctx.style_mut(|style| {
            style.visuals.window_shadow = Shadow::NONE;
        });
    }
}
