use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Hunger(f32);

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Thirst(f32);

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	player: Player,
	hunger: Hunger,
	thirst: Thirst,
}

impl Default for PlayerBundle {
	fn default() -> Self { 
		Self {
			player: Player,
			hunger: Hunger(1.0),
			thirst: Thirst(1.0),
		}
	}
}

pub fn move_player(
	query: Query<&ActionState<crate::input::Action>, With<Player>>
) {
	// TODO
}

pub fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = meshes.add(shape::Capsule { radius: 0.4, depth: 1.0, ..default() }.into());
	let material = materials.add(StandardMaterial::default());

	commands.spawn((
		Name::new("Player"),
		PlayerBundle::default(),
		PbrBundle {
			mesh,
			material,
			..default()
		},
		crate::input::default_inputs(),
	)).with_children(|commands| {
		// TODO: Spawn camera
	});
}