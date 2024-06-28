use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (capture_wasd, move_wasd).chain())
        .add_systems(Update, (capture_mouse_clicks).chain())
        .run();
}

// TODO: I think this should be made into a global resource, rather than a per-entity component
#[derive(Component)]
struct WasdInput(Vec2);

#[derive(Component)]
struct ClickCapturer;

#[derive(Component)]
struct WasdMove {
    velocity: Vec2,
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        WasdInput(Vec2::ZERO),
        WasdMove { velocity: Vec2::ZERO, speed: 300. },
        ClickCapturer
    ));

    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(50., 100.)));
    let player = Mesh2dHandle(meshes.add(RegularPolygon::new(50., 5)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle,
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(0., 0., -10.),
        ..default()
    });
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: player,
            material: materials.add(Color::PURPLE),
            ..default()
        },
        WasdInput(Vec2::ZERO),
        WasdMove { velocity: Vec2::ZERO, speed: 300. }
    ));
}

fn capture_wasd(
    mut query: Query<&mut WasdInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut wasd_input in &mut query {
        wasd_input.0 = Vec2::ZERO;

        // get direction
        if keyboard.pressed(KeyCode::KeyA) {
            wasd_input.0.x -= 1.;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            wasd_input.0.x += 1.;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            wasd_input.0.y += 1.;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            wasd_input.0.y -= 1.;
        }

        // normalise
        wasd_input.0 = wasd_input.0.normalize_or_zero();
    }
}

fn move_wasd(
    mut query: Query<(&mut Transform, &mut WasdMove, &WasdInput)>,
    time: Res<Time>,
) {
    for (mut transform, mut wasd_move, input) in &mut query {
        wasd_move.velocity = input.0 * wasd_move.speed;

        transform.translation.x += wasd_move.velocity.x * time.delta_seconds();
        transform.translation.y += wasd_move.velocity.y * time.delta_seconds();
    }
}

fn capture_mouse_clicks(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<ClickCapturer>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse.just_pressed(MouseButton::Left) {
            info!("Mouse click at world position ({},{})", world_position.x, world_position.y);
        }
    }
}
