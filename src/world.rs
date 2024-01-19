use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::interaction::TestInteraction;

pub fn spawn_world(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(shape::Plane { size: 15.0, ..default() }.into());
	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Main plane"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(Vec3::NEG_Y),
			..default()
		},
		RigidBody::Static,
		Collider::heightfield(vec![vec![0.0;2];2], Vec3::splat(15.0)),
		TestInteraction,
	));


	commands.spawn((
		Name::new("Light"),
		DirectionalLightBundle {
			directional_light: DirectionalLight { 
				illuminance: 30000.0,
				..default()
			},
			transform: Transform::from_rotation(Quat::from_euler(
				EulerRot::XYZ,
				1.5,
				3.3,
				-2.1,
			)),
			..default()
		}
	));

	// Ambient light
	commands.insert_resource(AmbientLight {
		color: Color::ALICE_BLUE,
		brightness: 0.15,
	});
}