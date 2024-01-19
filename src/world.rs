use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub fn spawn_world(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(shape::Plane { size: 10.0, ..default() }.into());
	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Main plane"),
		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::NEG_Y),
			..default()
		},
		// RigidBody::Static,
		// Collider::heightfield()
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