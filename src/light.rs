use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}

fn spawn_lights(mut commands: Commands) {
    let ambient_light = AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    };

    commands.insert_resource(ambient_light);

    let ligth = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        Name::new("MainLight"),
    );

    commands.spawn(ligth);
}
