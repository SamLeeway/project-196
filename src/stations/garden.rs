use crate::{
	interaction::StationInteractionEvent,
	items::{Cup, Growable, ItemHolder},
	player::Player,
};
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GardenPlot {
	pub water: f32,
}

// #[derive(Component, Debug, Clone)]
// pub struct Planter {
// 	seeder: bool,
// 	watering: bool,
// 	//harvester: bool,
// 	//fertilized: bool,
// }

pub fn check_events_garden(
	_commands: Commands,
	mut event_reader: EventReader<StationInteractionEvent>,
	mut plot_query: Query<(&mut GardenPlot, &mut ItemHolder), Without<Player>>,
	mut inventory_query: Query<&mut ItemHolder, With<Player>>,
	// Items
	mut cup_query: Query<&mut Cup>,
) {
	for event in event_reader.read() {
		let Ok((mut plot, mut plot_holder)) = plot_query.get_mut(event.station) else {
			continue;
		};

		let Ok(mut player_holder) = inventory_query.get_mut(event.player) else {
			continue;
		};

		if let Some(item) = player_holder.item {
			if let Ok(mut cup) = cup_query.get_mut(item) {
				if cup.filled {
					cup.filled = false;
					plot.water = (plot.water + 10.0).min(150.0);
				}
			} else if plot_holder.is_none() {
				plot_holder.item = std::mem::take(&mut player_holder.item);
			}
		} else if plot_holder.is_some() {
			player_holder.item = std::mem::take(&mut plot_holder.item);
		}
	}
}

pub fn tick_garden(
	mut plot_query: Query<(Entity, &mut GardenPlot, &ItemHolder)>,
	mut growable_query: Query<(&mut Growable, &mut Transform)>,
	time: Res<Time>,
) {
	for (_plot_entity, mut plot, holder) in plot_query.iter_mut() {
		if let Some(planted_item) = holder.item {
			if let Ok((mut growable, mut transform)) = growable_query.get_mut(planted_item) {
				if growable.growth < 100.0 && plot.water > 0.0 {
					growable.growth = (growable.growth + time.delta_seconds() * 2.0).min(100.0);
					transform.scale = Vec3::splat((growable.growth / 100.0) * 0.3 + 0.1);
					plot.water = (plot.water - time.delta_seconds() / 2.0).max(0.0);
				}
			}
		}
	}
}
