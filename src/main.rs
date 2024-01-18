#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use leafwing_input_manager::prelude::*;

mod player;
mod world;
mod input;

fn main() {
    let mut app = App::default();

    app .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Project 196".into(),
                    ..default()
                }),
                ..default()
            }),
            InputManagerPlugin::<input::Action>::default()
        ))
        .add_systems(Startup, (
            crate::player::spawn_player, 
            crate::world::spawn_world
        ));

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
