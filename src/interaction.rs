use std::mem;

use bevy::{
	ecs::{change_detection::MutUntyped, event::ManualEventReader, system::SystemState},
	prelude::*,
	reflect::{ReflectFromPtr, TypeRegistry},
};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
	input::Action, items::*, player::{Player, PlayerCamera, Hunger, Thirst, Inventory}, stations::*
};

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			interact_input
				.after(crate::player::move_player),
		)
		.add_event::<ItemInteractionEvent>()
		.add_event::<StationInteractionEvent>()
		.add_systems(Update, (
			check_events_filter,
			tick_filter,
			check_events_shelf,
			check_events_cup,
		));
	}
}

#[derive(Event, Debug, Clone)]
pub struct ItemInteractionEvent {
	pub player: Entity,
	pub item: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct StationInteractionEvent {
	pub player: Entity,
	pub item: Option<Entity>,
	pub station: Entity,
}

#[derive(PhysicsLayer)]
pub enum Layer {
	Player,
	Interactable,
	Ground,
}

pub fn interact_input(
	player_query: Query<(Entity, &ActionState<Action>, &Children, &Inventory), With<Player>>,
	camera_query: Query<&GlobalTransform, With<PlayerCamera>>,
	mut item_event_writer: EventWriter<ItemInteractionEvent>,
	mut station_event_writer: EventWriter<StationInteractionEvent>,
	spatial_query: SpatialQuery,
) {
	for (player, input, children, inventory) in player_query.iter() {
		if !input.just_pressed(Action::Use) {
			continue;
		}

		let camera_entity = children
			.iter()
			.find(|entity| camera_query.contains(**entity))
			.unwrap();

		let mut camera_transform = camera_query.get(*camera_entity).unwrap();

		let filter = SpatialQueryFilter::new().without_entities([player]).with_masks([Layer::Interactable]);

		if let Some(ray_hit) = spatial_query.cast_ray(
			camera_transform.translation(),
			camera_transform.forward(),
			4.0,
			true,
			filter,
		) {
			station_event_writer.send(StationInteractionEvent {
				player,
				item: inventory.main_hand,
				station: ray_hit.entity,
			});
		} else if let Some(item) = inventory.main_hand {
			item_event_writer.send(ItemInteractionEvent { 
				player, 
				item,
			});
		}
	
	}
}