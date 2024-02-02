use bevy::prelude::*;

use crate::{
	energy::ElecticComponent,
	interaction::StationInteractionEvent,
	items::Cup,
	player::{Player, Thirst},
};

#[derive(Component, Debug, Clone)]
pub struct WaterFilter {
	//ty: FilterType,
	pub filtered_water: f32,
	pub is_broken: bool,
}

// #[derive(Debug, Clone, Copy)]
// pub enum FilterType {
// 	GoodFilter,
// 	PrimitiveFilter,
// 	Unfiltered,
// }

pub fn check_events_filter(
	mut event_reader: EventReader<StationInteractionEvent>,
	mut filter_query: Query<&mut WaterFilter>,
	mut player_query: Query<&mut Thirst, With<Player>>,

	mut cup_query: Query<&mut Cup>,
) {
	for event in event_reader.read() {
		let Ok(mut filter) = filter_query.get_mut(event.station) else {
			continue;
		};

		let Ok(mut thirst) = player_query.get_mut(event.player) else {
			continue;
		};

		if let Some(mut cup) = event.item.and_then(|item| cup_query.get_mut(item).ok()) {
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

pub fn tick_filter(mut filter_q: Query<(&mut WaterFilter, &ElecticComponent)>, time: Res<Time>) {
	for (mut filter, component) in filter_q.iter_mut() {
		if component.powered {
			filter.filtered_water += time.delta_seconds() * 2.0;
		}
	}
}
