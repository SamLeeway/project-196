use bevy::prelude::*;

use crate::{interaction::ItemInteractionEvent, player::{Player, Thirst}};

#[derive(Component, Debug, Clone, Copy)]
pub struct Wrench;

#[derive(Component, Debug, Clone, Copy)]
pub struct Cup {
	pub filled: bool
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Carrot {
	growth: f32,
}

pub fn check_events_cup(
	mut event_reader: EventReader<ItemInteractionEvent>,
	mut cup_query: Query<&mut Cup>,
	mut player_query: Query<&mut Thirst, With<Player>>,
) {
	for event in event_reader.read() {
		let Ok(mut cup) = cup_query.get_mut(event.item) else {
			continue;
		};

		let Ok(mut thirst) = player_query.get_mut(event.player) else {
			continue;
		};

		if cup.filled {
			cup.filled = false;
			thirst.0 += 10.0;

			#[cfg(debug_assertions)]
			println!("Drank the water from the cup. Thirst: {:.0}", thirst.0);
		} else {
			#[cfg(debug_assertions)]
			println!("Cup is empty. Nothing happens");

		}
	}
}