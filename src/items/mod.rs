use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
	interaction::ItemInteractionEvent,
	player::{Player, Thirst},
};

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				(check_holders, apply_deferred, update_position).chain(),
				check_events_cup,
			),
		);
	}
}

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct ItemHolder {
	pub item: Option<Entity>,
}

#[derive(Component, Debug, Clone)]
pub struct ItemDisplay {
	pub display_root: Option<Entity>,
	pub ty: DisplayType,
}

#[derive(Component, Debug, Clone)]
pub struct ItemDisplaySettings {
	pub map: HashMap<DisplayType, (Vec3, Quat)>,
}

/// Used to set the position of how item is displayed in different places.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayType {
	Hand,
	Lying, // On the ground or on shelf
	Planted,
}

/// If location of the item changes, change it's parent to display entity
fn check_holders(
	mut commands: Commands,
	query: Query<(Entity, &ItemHolder, &ItemDisplay), Changed<ItemHolder>>,
) {
	for (entity, holder, display) in query.iter() {
		if let Some(item) = holder.item {
			commands
				.entity(display.display_root.unwrap_or(entity))
				.add_child(item);
		}
	}
}

/// If parent of an item changes, update it's local position
fn update_position(
	item_display_query: Query<&ItemDisplay>,
	mut transferred_item_query: Query<
		(&Parent, &ItemDisplaySettings, &mut Transform),
		Or<(Changed<Parent>, Changed<ItemDisplaySettings>)>,
	>,
) {
	for (parent, settings, mut transform) in transferred_item_query.iter_mut() {
		let Ok(display) = item_display_query.get(parent.get()) else {
			continue;
		};

		let (translation, rotation) = settings.map.get(&display.ty).copied().unwrap_or_default();

		transform.translation = translation;
		transform.rotation = rotation;
	}
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Wrench;

#[derive(Component, Debug, Clone, Copy)]
pub struct Cup {
	pub filled: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Carrot;

#[derive(Component, Debug, Clone, Copy)]
pub struct Growable {
	pub growth: f32,
}

pub fn check_events_cup(
	mut event_reader: EventReader<ItemInteractionEvent>,
	mut cup_query: Query<&mut Cup>,
	mut player_query: Query<&mut Thirst, With<Player>>,
) {
	for event in event_reader.read() {
		let Ok(mut cup) = cup_query.get_mut(event.item) else {
			continue;
		};

		let Ok(mut thirst) = player_query.get_mut(event.player) else {
			continue;
		};

		if cup.filled {
			cup.filled = false;
			thirst.0 += 10.0;

			#[cfg(debug_assertions)]
			println!("Drank the water from the cup. Thirst: {:.0}", thirst.0);
		} else {
			#[cfg(debug_assertions)]
			println!("Cup is empty. Nothing happens");
		}
	}
}
