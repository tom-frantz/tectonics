use crate::camera::CameraPlugin;
use crate::render::RenderPlugin;
use crate::ui::theme::{Colour, MenuColour};
use crate::world::HexWorld;
use bevy::{prelude::*, ui::UiPlugin as BevyUiPlugin};

mod camera;
mod render;
mod ui;
mod world;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(MenuColour::BlackPen.color()))
        .add_startup_system(init)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Fucking Tectonics".to_string(),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(BevyUiPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CameraPlugin)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(HexWorld::new());
}
