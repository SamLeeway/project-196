use bevy::prelude::*;
use crate::{interaction::InteractionEvent, player::{Thirst, Player, Hunger}, items::Item};
use bevy_xpbd_3d::prelude::*;
use std::ops::AddAssign;

#[derive(Component, Debug, Clone)]
pub enum Station {
	WaterStation(WaterStation),
	Shelf(Option<Entity>),
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub enum WaterStation {
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
	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Water station"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0),
			..default()
		},
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		Station::WaterStation(WaterStation::GoodFilter),
	));

	commands.spawn((
		Name::new("Shelf"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::X * 4.0 + Vec3::Z * 2.0),
			..default()
		},
		RigidBody::Static,
		Collider::cuboid(1.0,1.0,1.0),
		Station::Shelf(None),
	));
}