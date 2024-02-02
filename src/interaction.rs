use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
	input::Action,
	items::*,
	player::{Player, PlayerCamera},
};

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, interact_input.after(crate::player::move_player))
			.add_event::<ItemInteractionEvent>()
			.add_event::<StationInteractionEvent>();
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
	player_query: Query<(Entity, &ActionState<Action>, &Children, &ItemHolder), With<Player>>,
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

		let camera_transform = camera_query.get(*camera_entity).unwrap();

		let filter = SpatialQueryFilter::new()
			.without_entities([player])
			.with_masks([Layer::Interactable]);

		if let Some(ray_hit) = spatial_query.cast_ray(
			camera_transform.translation(),
			camera_transform.forward(),
			4.0,
			true,
			filter,
		) {
			station_event_writer.send(StationInteractionEvent {
				player,
				item: inventory.item,
				station: ray_hit.entity,
			});
		} else if let Some(item) = inventory.item {
			item_event_writer.send(ItemInteractionEvent { player, item });
		}
	}
}
