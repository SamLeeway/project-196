use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
	Look,
    Jump,
	Use,
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
			.build(),
	}
}
