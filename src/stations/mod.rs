use bevy::prelude::*;
use crate::{energy::{ElecticComponent, EnergyGenerator, TurnedOn}, interaction::{Layer, StationInteractionEvent}, items::{Cup, Wrench}, player::{Hunger, Inventory, ItemParent, Player, Thirst}};
use bevy_xpbd_3d::prelude::*;
use std::{f32::consts::PI, ops::AddAssign};

#[derive(Component, Debug, Clone)]
pub struct Planter {
	//growing_plant: Plant,
	seeder: bool,
	watering: bool,
	//harvester: bool,
	//fertilized: bool,
	current_water: f32,
	growth_progress: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Shelf(Option<Entity>);

#[derive(Component, Debug, Clone)]
pub struct 	WaterFilter {
	//ty: FilterType,
	filtered_water: f32,
	is_broken: bool,
}


#[derive(Debug, Clone, Copy)]
pub enum FilterType {
	GoodFilter,
	PrimitiveFilter,
	Unfiltered,
}

pub fn spawn_station(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(
		shape::Cube { size: 1.0 }
		.into(),
	);
	let shelf_material = materials.add(StandardMaterial {
		alpha_mode: AlphaMode::Premultiplied,
		base_color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.5 },
		..default()
	});

	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Water station"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0),
			..default()
		},
		TurnedOn(true),
		ElecticComponent {
			powered: false,
			consumption: 3.0,
		},
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		WaterFilter {
    		filtered_water: 0.0,
    		is_broken: false,
		},
	));

	commands.spawn((
		Name::new("Generator"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0 + Vec3::Z * -3.0),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		EnergyGenerator { 
			generates: 5.0,
		},
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		Shelf(None),
	));

	let cup = commands.spawn((
		Name::new("Cup"),
		Cup { filled: false },
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(0.2)),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		RigidBody::Kinematic,
		Collider::cuboid(1.0,1.0,1.0),
	)).id();

	commands.spawn((
		Name::new("Shelf"),
		PbrBundle {
			mesh: mesh.clone(),
			material: shelf_material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0 + Vec3::Z * 2.0),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		Shelf(Some(cup)),
	)).add_child(cup);;

	let wrench = commands.spawn((
		Name::new("Wrench"),
		Wrench,
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::ZERO).with_rotation(Quat::from_rotation_z(PI/4.0)).with_scale(Vec3::splat(0.2)),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		RigidBody::Kinematic,
		Collider::cuboid(1.0,1.0,1.0),
	)).id();

	commands.spawn((
		Name::new("Shelf 2"),
		PbrBundle {
			mesh: mesh.clone(),
			material: shelf_material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0 + Vec3::Z * 4.0),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		Shelf(Some(wrench)),
	)).add_child(wrench);
}

pub fn check_events_filter(
	mut event_reader: EventReader<StationInteractionEvent>,
	mut filter_q: Query<&mut WaterFilter>,
	mut cup_q: Query<&mut Cup>,
	mut player_q: Query<&mut Thirst, With<Player>>,
) {
	for event in event_reader.read() {
		let Ok(mut filter) = filter_q.get_mut(event.station) else {
			continue;
		};

		let Ok(mut thirst) = player_q.get_mut(event.player) else {
			continue;
		};

		if let Some(mut cup) = event.item.and_then(|item| cup_q.get_mut(item).ok()) {
			if cup.filled {
				#[cfg(debug_assertions)]
				println!("Poured the water in the filter");

				cup.filled = false;
				filter.filtered_water += 10.0;
			} else if filter.filtered_water > 10.0 {
				#[cfg(debug_assertions)]
				println!("Filled the cup from the filter");
				
				filter.filtered_water -= 10.0;
				cup.filled = true;
			} else {
				#[cfg(debug_assertions)]
				println!("Not enough water in the filter");
			}
		} else if filter.filtered_water > 10.0 {
			filter.filtered_water -= 10.0;
			thirst.0 += 10.0;

			#[cfg(debug_assertions)]
			println!("Drank the water from the filter. Thirst: {:.0}", thirst.0);
		} else {
			#[cfg(debug_assertions)]
			println!("Not enough water in the filter");
		}
	}
}

pub fn tick_filter(	
	mut filter_q: Query<(&mut WaterFilter, &ElecticComponent)>,
	time: Res<Time>,
) {
	for (mut filter, component) in filter_q.iter_mut() {
		if component.powered {
			filter.filtered_water += time.delta_seconds() * 2.0;
		}
	}
}

pub fn check_events_shelf(
	mut commands: Commands,
	mut event_reader: EventReader<StationInteractionEvent>,
	mut shelf_query: Query<(Entity, &mut Shelf)>,
	mut player_query: Query<&mut Inventory, With<Player>>,
	mut item_parent_query: Query<Entity, With<ItemParent>>,
	#[cfg(debug_assertions)]
	mut name: Query<&Name>,
) {
	for event in event_reader.read() {
		let Ok(mut shelf) = shelf_query.get_mut(event.station) else {
			continue;
		};

		let Ok(mut inventory) = player_query.get_mut(event.player) else {
			continue;
		};


		#[cfg(debug_assertions)]
		match (inventory.main_hand, shelf.1.0) {
			(Some(hand), Some(shelf)) => {
				let hand = name.get(hand).map(|x| x.as_str()).unwrap_or("Item");
				let shelf = name.get(shelf).map(|x| x.as_str()).unwrap_or("Item");
				println!("Put {} on the shelf and got {} from the shelf", hand, shelf);
			},
			(Some(hand), None) => {
				let hand = name.get(hand).map(|x| x.as_str()).unwrap_or("Item");
				println!("Put {} on the shelf", hand);
			},
			(None, Some(shelf)) => {
				let shelf = name.get(shelf).map(|x| x.as_str()).unwrap_or("Item");
				println!("Got {} from the shelf", shelf);
			},
			(None, None) => (),
		}

		if let Some(hand_item) = inventory.main_hand {
			commands.entity(hand_item).set_parent(shelf.0);
		}

		if let Some(shelf_item) = shelf.1.0 {
			// TODO: Make item parent query GOOD :)
			commands.entity(shelf_item).set_parent(item_parent_query.single());
		}

		std::mem::swap(&mut inventory.main_hand, &mut shelf.1.0);


	}
}