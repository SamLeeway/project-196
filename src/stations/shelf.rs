use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{
	interaction::{Layer, StationInteractionEvent},
	items::{DisplayType, ItemDisplay, ItemHolder},
	player::Player,
};

#[derive(Component, Debug, Clone)]
pub struct Shelf;

pub fn check_events_shelf(
	_commands: Commands,
	mut event_reader: EventReader<StationInteractionEvent>,
	mut shelf_query: Query<&mut ItemHolder, (With<Shelf>, Without<Player>)>,
	mut player_query: Query<&mut ItemHolder, With<Player>>,
) {
	for event in event_reader.read() {
		let Ok(mut shelf_holder) = shelf_query.get_mut(event.station) else {
			continue;
		};
		let Ok(mut player_holder) = player_query.get_mut(event.player) else {
			continue;
		};

		std::mem::swap(shelf_holder.as_mut(), player_holder.as_mut());
	}
}

pub fn spawn_empty_rack(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
	spawn_filled_rack(commands, meshes, materials, &[])
}

pub fn spawn_filled_rack(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	items: &[Entity],
) -> Entity {
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

	commands.spawn((
		Name::new("Rack"),
		TransformBundle::default(),
		VisibilityBundle::default(),
		// Import model later
	)).with_children(|commands| {
		for y in 0..2 {
			for x in 0..3 {
				commands.spawn((
					Name::new("Shelf"),
					PbrBundle {
						mesh: mesh.clone(),
						material: shelf_material.clone(),
						transform: Transform::from_xyz(x as f32, y as f32, 0.0),
						..default()
					},
					CollisionLayers::new([Layer::Interactable], []),
					RigidBody::Static,
					Collider::cuboid(1.0, 1.0, 1.0),
					Shelf,
					ItemHolder { item: items.get(x + y * 3).copied() },
					ItemDisplay {
						ty: DisplayType::Lying,
						display_root: None,
					},
				));
			}
		}
	}).id()
}