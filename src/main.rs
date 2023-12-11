use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

mod camera;
mod light;
mod player;
mod world;

use camera::CameraPlugin;
use light::LightPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CameraPlugin,
            ThirdPersonCameraPlugin,
            WorldPlugin,
            LightPlugin,
            PlayerPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
