use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (setup_scene_once_loaded, player_movement));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed {
    value: f32,
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn spawn_player(mut commands: Commands, asset: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                intensity: 2500.0,
                color: Color::rgb(1.0, 0.99, 0.71),
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 3.5, 2.5),
                rotation: Quat::from_rotation_x(180.0),
                ..default()
            },
            ..default()
        },
        Name::new("Flashlight"),
    );

    let player = (
        SceneBundle {
            scene: asset.load("models/robot/scene.gltf#Scene0"),
            ..default()
        },
        Speed { value: 2.5 },
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands
        .spawn(player)
        .with_children(|parent| {
            parent.spawn(flashlight);
        })
        .insert((
            RigidBody::Dynamic,
            Collider::cuboid(2.0, 1.0, 2.0),
            TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 8.0),
                ..default()
            },
        ));

    commands.insert_resource(Animations(vec![
        asset.load("models/robot/scene.gltf#Animation0")
    ]));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for mut player in animation_players.iter_mut() {
        if keys.just_pressed(KeyCode::K) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }
    }

    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.value * time.delta_seconds();
        player_transform.translation += movement;

        if direction.length_squared() > 0.0 {
            player_transform.look_to(-direction, Vec3::Y)
        }
    }
}

// fn spawn_player(mut commands: Commands, asset: Res<AssetServer>) {
//     let flashlight = SpotLightBundle::default();

//     let player = (
//         SceneBundle {
//             scene: asset.load("models/leo-rover/scene.gltf#Scene0"),
//             transform: Transform::from_xyz(0.0, -2.0, 0.0),
//             ..default()
//         },
//         Speed { value: 2.5 },
//         Player,
//         ThirdPersonCameraTarget,
//     );

//     commands.spawn(player).with_children(|parent| {
//         parent.spawn(flashlight);
//     });
// }
