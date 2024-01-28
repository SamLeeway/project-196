use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct TurnedOn(pub bool);

#[derive(Component, Debug, Clone, Copy)]
pub struct ElecticComponent {
	pub powered: bool,
	pub consumption: f32, // Per second
}

#[derive(Component, Debug, Clone, Copy)]
pub struct EnergyGenerator {
	pub generates: f32, // Per second
}

pub fn energy_system(
	generator_q: Query<(&TurnedOn, &EnergyGenerator)>,
	mut consumer_q: Query<(&TurnedOn, &mut ElecticComponent)>,

) {
	let generated_power: f32 = generator_q.iter().filter(|q| **q.0).map(|q| q.1.generates).sum();
	let consumed_power: f32 = consumer_q.iter().filter(|q| **q.0).map(|q| q.1.consumption).sum();

	let enough_power = consumed_power > generated_power;

	for (turned_on, mut component) in consumer_q.iter_mut() {
		component.powered = turned_on.0 && enough_power;
	}
}

