use bevy::prelude::*;
use bevy_inspector_egui::quick::*;

fn main() {
    let mut app = App::default();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Project 196".into(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
