use std::mem;

use bevy::{
	ecs::{change_detection::MutUntyped, event::ManualEventReader, system::SystemState},
	prelude::*,
	reflect::{ReflectFromPtr, TypeRegistry},
};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
	input::Action,
	player::{Player, PlayerCamera, Hunger, Thirst, Inventory}, stations::Station, items::Item,
};

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(interact_input, interact)
				.chain()
				.after(crate::player::move_player),
		)
		.add_event::<InteractionEvent>();
	}
}

#[derive(Event, Debug, Clone)]
pub struct InteractionEvent {
	pub player: Entity,
	pub station: Option<Entity>,
}

#[derive(PhysicsLayer)]
enum Layer {
	Player,
	Interactable,
	Ground,
}

pub fn interact_input(
	player_query: Query<(Entity, &ActionState<Action>, &Children), With<Player>>,
	camera_query: Query<&GlobalTransform, With<PlayerCamera>>,
	mut event_writer: EventWriter<InteractionEvent>,
	spatial_query: SpatialQuery,
) {
	for (entity, input, children) in player_query.iter() {
		if !input.just_pressed(Action::Use) {
			continue;
		}

		let camera_entity = children
			.iter()
			.find(|entity| camera_query.contains(**entity))
			.unwrap();

		let mut camera_transform = camera_query.get(*camera_entity).unwrap();

		let filter = SpatialQueryFilter::new().without_entities([entity]); //.with_masks([Layer::Interactable]);

		let ray_hit = spatial_query.cast_ray(
			camera_transform.translation(),
			camera_transform.forward(),
			4.0,
			true,
			filter,
		);
		
		event_writer.send(InteractionEvent {
			player: entity,
			station: ray_hit.map(|hit| hit.entity),
		});
	}
}


pub fn interact(
	mut event_reader: EventReader<InteractionEvent>,
	mut player_q: Query<(&mut Hunger, &mut Thirst, &mut Inventory), With<Player>>,
	mut item_q: Query<&mut Item>,
	mut station_q: Query<&mut Station>,
) {
	for InteractionEvent { player, station } in event_reader.read() {

		let (mut hunger, mut thirst, mut inventory) = player_q.get_mut(*player).unwrap();
	
		let mut main_item = inventory.main_hand.and_then(|entity| item_q.get_mut(entity).ok());
		let mut station = station.and_then(|entity| station_q.get_mut(entity).ok());

		match (main_item.as_deref_mut(), station.as_deref_mut()) {
			(Some(Item::Cup { filled }), Some(Station::WaterStation(ty))) => {
				if !*filled {
					println!("Filled cup!");
					*filled = true;
				}
			},
			(hand_item, Some(Station::Shelf(shelf_item))) => { 
				if let Some(hand_item) = hand_item {
					println!("Put {:?} on the shelf", hand_item);
				} 
				if let Some(shelf_item) = shelf_item.and_then(|entity| item_q.get(entity).ok()) {
					println!("Got {:?} from the shelf", shelf_item);
				}
				mem::swap(&mut inventory.main_hand, shelf_item);
			},
			(_, Some(Station::WaterStation(ty))) => {
				println!("Drank from filter!");
				thirst.0 += 10.0;
			},
			(Some(Item::Cup { filled }), _) => {
				if *filled {
					println!("Drank from cup!");
					*filled = false;
					thirst.0 += 10.0;
				}
			},
			_ => (),
		}
	}
}
