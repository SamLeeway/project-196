use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(InputManagerPlugin::<Action>::default())
			.add_systems(Update, cursor_grab_system);
	}
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
	Move,
	Look,
	Jump,
	Use,
	UnlockMouse,
}

pub fn default_inputs() -> InputManagerBundle<Action> {
	InputManagerBundle::<Action> {
		action_state: ActionState::default(),
		input_map: InputMap::default()
			.insert(DualAxis::left_stick(), Action::Move)
			.insert(VirtualDPad::wasd(), Action::Move)
			.insert(QwertyScanCode::E, Action::Use)
			.insert(MouseButton::Left, Action::Use)
			.insert(GamepadButtonType::RightTrigger2, Action::Use)
			.insert(DualAxis::right_stick(), Action::Look)
			.insert(DualAxis::mouse_motion(), Action::Look)
			.insert(QwertyScanCode::Escape, Action::UnlockMouse)
			.build(),
	}
}

pub fn cursor_grab_system(
	mut window: Query<&mut Window, With<PrimaryWindow>>,
	input: Query<&ActionState<Action>>,
	#[cfg(debug_assertions)] mut gui: Query<&mut bevy_inspector_egui::bevy_egui::EguiContext>,
) {
	let mut window = window.single_mut();

	let Ok(input) = input.get_single() else {
		return;
	};

	#[cfg(debug_assertions)]
	let cursor_over_ui = gui.single_mut().get_mut().is_pointer_over_area();

	#[cfg(not(debug_assertions))]
	let cursor_over_ui = false;

	if input.just_pressed(Action::Use) && !cursor_over_ui {
		window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
		window.cursor.visible = false;
	}

	if input.just_pressed(Action::UnlockMouse) {
		window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
		window.cursor.visible = true;
	}
}
