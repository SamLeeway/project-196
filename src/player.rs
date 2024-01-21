use std::{f32::consts::PI, ops::DerefMut};

use bevy::{
	prelude::*,
	window::{CursorGrabMode, PrimaryWindow},
};
use bevy_xpbd_3d::{parry::na::clamp, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{input::Action, items::Item};


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
				Startup,
				(crate::player::spawn_player, crate::world::spawn_world),
			)	
			.add_systems(
				Update,
				(crate::player::move_player, crate::player::drain_stats),
			)
			.register_type::<Health>()
			.register_type::<Hunger>()
			.register_type::<Thirst>()
			.register_type::<Energy>()
			.register_type::<Inventory>();
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Hunger(pub f32);

#[derive(Component, Reflect, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Thirst(pub f32);

#[derive(Component, Reflect, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Energy(pub f32);

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
pub struct Inventory {
	pub main_hand: Option<Entity>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerBundle {
	player: Player,
	health: Health,
	hunger: Hunger,
	energy: Energy,
	thirst: Thirst,
	inventory: Inventory,
}

impl Default for PlayerBundle {
	fn default() -> Self {
		Self {
			player: Player,
			health: Health(1.0),
			hunger: Hunger(1.0),
			thirst: Thirst(1.0),
			energy: Energy(1.0),
			inventory: Inventory::default(),
		}
	}
}

pub fn move_player(
	mut player_query: Query<
		(Entity, &ActionState<Action>, &mut Transform, &Children),
		With<Player>,
	>,
	mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
	window: Query<&Window, With<PrimaryWindow>>,
	mut time: Res<Time>,
) {
	for (entity, inputs, mut player_transform, children) in player_query.iter_mut() {
		let camera_entity = children
			.iter()
			.find(|entity| camera_query.contains(**entity))
			.unwrap();
		let mut camera_transform = camera_query.get_mut(*camera_entity).unwrap();

		let window = window.single();

		// Camera controls
		if matches!(window.cursor.grab_mode, CursorGrabMode::Locked) {
			let look_input = inputs.axis_pair(Action::Look).unwrap_or_default().xy() * 0.005;
			// Rotating camera up/down
			let (_, camera_x_rot, _) = camera_transform.rotation.to_euler(EulerRot::default());
			let camera_x_rot = clamp(camera_x_rot - look_input.y, -PI / 2.0, PI / 2.0);

			camera_transform.rotation =
				Quat::from_euler(EulerRot::default(), 0.0, camera_x_rot, 0.0);
			// Rotating character left/right
			player_transform.rotate_axis(Vec3::Y, -look_input.x);
		}

		// Moving character
		let mut move_input = inputs.axis_pair(Action::Move).unwrap_or_default().xy();
		move_input.y = -move_input.y;

		if move_input.length_squared() > 1.0 {
			move_input = move_input.normalize_or_zero();
		}

		player_transform.translation = player_transform.translation
			+ player_transform.rotation * move_input.extend(0.0).xzy() * time.delta_seconds() * 6.0;
	}
}

pub fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(
		shape::Capsule {
			radius: 0.4,
			depth: 1.0,
			..default()
		}
		.into(),
	);
	let material = materials.add(StandardMaterial::default());

	let item = commands.spawn((
		Name::new("Cup"),
		Item::Cup { filled: false },
	)).id();

	commands
		.spawn((
			Name::new("Player"),
			PlayerBundle {
				inventory: Inventory { main_hand: Some(item) },
				..default()
			},
			PbrBundle {
				mesh,
				material,
				..default()
			},
			crate::input::default_inputs(),
			RigidBody::Kinematic,
			LockedAxes::ROTATION_LOCKED,
			Collider::capsule(1.0, 0.4),
			LinearVelocity::default(),
		))
		.with_children(|commands| {
			commands.spawn((
				bevy::core_pipeline::fxaa::Fxaa::default(),
				Name::new("Player Camera"),
				PlayerCamera,
				Camera3dBundle {
					transform: Transform::from_translation(Vec3::Y * 0.75),
					projection: Projection::Perspective(PerspectiveProjection {
						fov: 85.0f32.to_radians(),
						..default()
					}),
					..default()
				},
				VisibilityBundle::default(),
			));
		});
}

pub fn drain_stats(
	mut hunger_q: Query<(Entity, &mut Hunger)>,
	mut energy_q: Query<(Entity, &mut Energy)>,
	mut thirst_q: Query<(Entity, &mut Thirst)>,
	mut health_q: Query<&mut Health>,
	time: Res<Time>,
) {
	for (entity, mut hunger) in hunger_q.iter_mut() {
		**hunger -= time.delta_seconds() / 120.0;
		if **hunger < 0.0 {
			health_q.get_mut(entity).unwrap().0 += **hunger;
			**hunger = 0.0;
		}
	}

	for (entity, mut thirst) in thirst_q.iter_mut() {
		**thirst -= time.delta_seconds() / 80.0;
		if **thirst < 0.0 {
			health_q.get_mut(entity).unwrap().0 += **thirst;
			**thirst = 0.0;
		}
	}
}
