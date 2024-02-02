#![allow(unused)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;

mod input;
mod interaction;
mod player;
mod ui;
mod world;

mod energy;
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
		ui::UiPlugin,
		PhysicsPlugins::default(),
		player::PlayerPlugin,
		items::ItemsPlugin,
		stations::StationsPlugin,
		interaction::InteractionPlugin,
	))
	.add_systems(Update, energy::energy_system);

	#[cfg(debug_assertions)]
	app.add_plugins(WorldInspectorPlugin::new());

	app.run();
}
