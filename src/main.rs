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
        .add_systems(Update, (capture_mouse_clicks, move_to_last_click).chain())
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

#[derive(Resource, Default)]
struct LastMouseClick(Vec2);

#[derive(Component)]
struct MoveToClicks {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.init_resource::<LastMouseClick>();

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
        MoveToClicks { speed: 300. }
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
    mut last_click: ResMut<LastMouseClick>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<ClickCapturer>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            last_click.0 = world_position;
        }
    }
}

fn move_to_last_click(
    last_click: Res<LastMouseClick>,
    mut query: Query<(&MoveToClicks, &mut Transform)>,
    time: Res<Time>,
) {
    for (mover, mut transform) in &mut query {
        let distance = transform.translation
            .truncate()
            .distance(last_click.0);

        // if they're already there, don't need to do anything
        if distance == 0. {
            break;
        }

        // if we can move there in one tick, do so
        if distance <= mover.speed * time.delta_seconds() {
            transform.translation.x = last_click.0.x;
            transform.translation.y = last_click.0.y;
            break;
        }

        // otherwise, step the maximum distance in the right direction
        let direction = last_click.0 - transform.translation.truncate();
        let step = direction.clamp_length_max(mover.speed * time.delta_seconds());
        transform.translation.x += step.x;
        transform.translation.y += step.y;
    }
}
