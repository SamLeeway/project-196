#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use leafwing_input_manager::prelude::*;

mod player;
mod world;
mod input;
mod interaction;

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
            input::InputPlugin,
            PhysicsPlugins::default(),
            interaction::InteractionPlugin,
        ))
        .add_systems(Startup, (
            crate::player::spawn_player, 
            crate::world::spawn_world
        ))
        .add_systems(Update, (
            crate::player::move_player,
            crate::player::drain_stats,
        ));

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
