#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use leafwing_input_manager::prelude::*;
use stations::WaterStation;

mod input;
mod interaction;
mod player;
mod world;

mod items;
mod stations;

fn main() {
	let mut app = App::default();

	app.add_plugins((
		DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Project 196".into(),
				..default()
			}),
			..default()
		}),
		input::InputPlugin,
		interaction::InteractionPlugin,
		PhysicsPlugins::default(),
		player::PlayerPlugin,
	))
	.add_systems(Startup, stations::spawn_station)
	.register_type::<WaterStation>();

	#[cfg(debug_assertions)]
	app.add_plugins(WorldInspectorPlugin::new());

	app.run();
}
