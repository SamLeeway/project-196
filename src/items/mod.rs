use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum Item {
	Wrench,
	Cup {
		filled: bool,
	}
}