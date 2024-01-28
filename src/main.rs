#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use leafwing_input_manager::prelude::*;
use stations::FilterType;

mod input;
mod interaction;
mod player;
mod world;
mod ui;

mod items;
mod stations;
mod energy;

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
		ui::UiPlugin,
	))
	.add_systems(Startup, stations::spawn_station)
	.add_systems(Update, energy::energy_system);

	#[cfg(debug_assertions)]
	app.add_plugins(WorldInspectorPlugin::new());

	app.run();
}