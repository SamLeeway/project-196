use bevy::{
	ecs::{change_detection::MutUntyped, event::ManualEventReader, system::SystemState},
	prelude::*,
	reflect::{ReflectFromPtr, TypeRegistry},
};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
	input::Action,
	player::{Player, PlayerCamera},
};

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(interact_input, interact)
				.chain()
				.after(crate::player::move_player),
		)
		.add_event::<InteractionEvent>()
		.register_type::<TestInteraction>();
	}
}

#[derive(Event, Debug, Clone, Copy)]
pub struct InteractionEvent {
	player: Entity,
	interactable: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Interactable;

// Quick and dirty interaction system cause I want to save my last two braincells
#[reflect_trait]
pub trait Interaction {
	fn interact(&mut self, interaction: InteractionEvent, world: &mut World);
}

#[derive(Component, Reflect)]
#[reflect(Interaction)]
pub struct TestInteraction;
impl Interaction for TestInteraction {
	fn interact(&mut self, interaction: InteractionEvent, world: &mut World) {
		println!("Touched some grass!");
	}
}

#[derive(PhysicsLayer)]
enum Layer {
	Player,
	Interactable,
	Ground,
}

pub fn interact_input(
	player_query: Query<(Entity, &ActionState<Action>, &Children), With<Player>>,
	camera_query: Query<&GlobalTransform, With<PlayerCamera>>,
	interactable_query: Query<Entity, Has<Interactable>>,
	mut event_writer: EventWriter<InteractionEvent>,
	spatial_query: SpatialQuery,
) {
	for (entity, input, children) in player_query.iter() {
		if !input.just_pressed(Action::Use) {
			continue;
		}

		let camera_entity = children
			.iter()
			.find(|entity| camera_query.contains(**entity))
			.unwrap();
		let mut camera_transform = camera_query.get(*camera_entity).unwrap();

		let filter = SpatialQueryFilter::new().without_entities([entity]); //.with_masks([Layer::Interactable]);

		if let Some(ray_hit) = spatial_query.cast_ray(
			camera_transform.translation(),
			camera_transform.forward(),
			5.0,
			true,
			filter,
		) {
			event_writer.send(InteractionEvent {
				player: entity,
				interactable: ray_hit.entity,
			})
		}
	}
}

// This is such a mess omfg
pub fn interact(world: &mut World) {
	let type_registry = world.resource::<AppTypeRegistry>().0.clone();
	let type_registry = type_registry.read();

	let mut events = world.resource_mut::<Events<InteractionEvent>>();
	let last_events: Vec<_> = events.get_reader().read(events.as_ref()).copied().collect();
	events.clear();

	for interaction in last_events {
		let components: Vec<_> = world
			.entity(interaction.interactable)
			.archetype()
			.components()
			.collect();

		for component_id in components.into_iter() {
			let type_id = world
				.components()
				.get_info(component_id)
				.unwrap()
				.type_id()
				.unwrap();

			let Some(component) = world.get_mut_by_id(interaction.interactable, component_id)
			else {
				continue;
			};

			let component: MutUntyped<'static> = unsafe { std::mem::transmute(component) };

			let Some(mut view) =
				get_interaction_from_mut_untyped(component, &type_registry, type_id)
			else {
				continue;
			};

			view.interact(interaction, world)
		}
	}
}

fn get_interaction_from_mut_untyped<'a>(
	component: MutUntyped<'a>,
	type_registry: &TypeRegistry,
	type_id: std::any::TypeId,
) -> Option<Mut<'a, dyn Interaction>> {
	let reflect_from_ptr = type_registry.get(type_id)?.data::<ReflectFromPtr>()?;
	let reflect_view = type_registry.get_type_data::<ReflectInteraction>(type_id)?;

	let reflect = component.map_unchanged(|ptr| unsafe { reflect_from_ptr.as_reflect_mut(ptr) });
	let view = reflect.map_unchanged(|reflect| unsafe {
		reflect_view.get_mut(std::mem::transmute(reflect)).unwrap()
	});

	Some(view)
}
