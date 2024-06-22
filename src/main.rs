use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (capture_wasd, move_wasd).chain())
        .run();
}

#[derive(Component)]
struct WasdInput(Vec2);

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
    commands.spawn(Camera2dBundle::default());

    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
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
