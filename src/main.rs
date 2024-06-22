use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle,
        material: materials.add(Color::PURPLE),
        ..default()
    });
}
