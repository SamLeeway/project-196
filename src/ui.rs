use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Startup, setup)
			.add_plugins(UiMaterialPlugin::<CursorMaterial>::default());
    }
}

pub fn setup(
	mut commands: Commands, 
	mut ui_materials: ResMut<Assets<CursorMaterial>>,
) {
	commands
	.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(100.0),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			flex_direction: FlexDirection::Column,
			..default()
		},
		..default()
	})
	.with_children(|parent| {
		parent.spawn(MaterialNodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				width: Val::Px(6.0),
				height: Val::Px(6.0),
				..default()
			},
			material: ui_materials.add(CursorMaterial {
				color: Color::WHITE.into(),
			}),
			..default()
		});

		parent.spawn(MaterialNodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				width: Val::Px(8.0),
				height: Val::Px(8.0),
				..default()
			},
			z_index: ZIndex::Global(-5),
			material: ui_materials.add(CursorMaterial {
				color: Color::BLACK.into(),
			}),
			..default()
		});
	});
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CursorMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for CursorMaterial {
    fn fragment_shader() -> ShaderRef {
        "cursor_shader.wgsl".into()
    }
}