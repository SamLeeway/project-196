use crate::{
	energy::{ElecticComponent, EnergyGenerator, TurnedOn},
	interaction::Layer,
	items::{Carrot, Cup, DisplayType, Growable, ItemDisplay, ItemHolder, Wrench},
};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;

mod shelf;
pub use shelf::Shelf;

mod garden;
pub use garden::GardenPlot;

mod filter;
pub use filter::WaterFilter;

pub struct StationsPlugin;

impl Plugin for StationsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				filter::tick_filter,
				filter::check_events_filter,
				garden::tick_garden,
				garden::check_events_garden,
				shelf::check_events_shelf,
			),
		)
		.add_systems(Startup, spawn_stations);
	}
}

pub fn spawn_stations(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(shape::Cube { size: 1.0 }.into());
	let shelf_material = materials.add(StandardMaterial {
		alpha_mode: AlphaMode::Premultiplied,
		base_color: Color::Rgba {
			red: 1.0,
			green: 1.0,
			blue: 1.0,
			alpha: 0.5,
		},
		..default()
	});

	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Water filter"),
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
		Collider::cuboid(1.0, 1.0, 1.0),
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
		EnergyGenerator { generates: 5.0 },
		RigidBody::Static,
		Collider::cuboid(1.0, 1.0, 1.0),
	));

	let cup = commands
		.spawn((
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
			Collider::cuboid(1.0, 1.0, 1.0),
		))
		.id();

	let wrench = commands
		.spawn((
			Name::new("Wrench"),
			Wrench,
			PbrBundle {
				mesh: mesh.clone(),
				material: material.clone(),
				transform: Transform::from_translation(Vec3::ZERO)
					.with_rotation(Quat::from_rotation_z(PI / 4.0))
					.with_scale(Vec3::splat(0.2)),
				..default()
			},
			CollisionLayers::new([Layer::Interactable], []),
			RigidBody::Kinematic,
			Collider::cuboid(1.0, 1.0, 1.0),
		))
		.id();

	let carrot = commands
		.spawn((
			Name::new("Carrot"),
			Carrot,
			Growable { growth: 0.0 },
			PbrBundle {
				mesh: mesh.clone(),
				material: material.clone(),
				transform: Transform::from_translation(Vec3::ZERO)
					.with_rotation(Quat::from_rotation_y(PI / 4.0))
					.with_scale(Vec3::splat(0.1)),
				..default()
			},
			CollisionLayers::new([Layer::Interactable], []),
			RigidBody::Kinematic,
			Collider::cuboid(1.0, 1.0, 1.0),
		))
		.id();


	let rack = shelf::spawn_filled_rack(&mut commands, &mut meshes, &mut materials, &[cup, wrench, carrot]);
	commands.entity(rack).insert(Transform::from_xyz(0.0, 0.0, 4.0).with_rotation(Quat::from_rotation_y(PI/2.0)));

	commands.spawn((
		Name::new("Plot"),
		GardenPlot { water: 30.0 },
		PbrBundle {
			mesh: mesh.clone(),
			material: shelf_material.clone(),
			transform: Transform::from_translation(Vec3::X * -4.0 + Vec3::Z * 0.0),
			..default()
		},
		CollisionLayers::new([Layer::Interactable], []),
		RigidBody::Static,
		Collider::cuboid(1.0, 1.0, 1.0),
		ItemHolder::default(),
		ItemDisplay {
			ty: DisplayType::Planted,
			display_root: None,
		},
	));
}
