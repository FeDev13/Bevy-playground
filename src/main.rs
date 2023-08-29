use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_player)
        .add_startup_system(camera)
        .run(); //llama a la funcion con add
}
/* pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Fede".to_string(),
    });
} //para spawnear entidades y agregarle o quitarle componentes, aca se crea una entidad con el componente person pero hay q darle una query para q interactue

pub fn mostrar_nombres(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Nombre:{}", person.name)
    }
}

#[derive(Component)] //el macro va arriba para indicar q la struct es un componente bevy

pub struct Person {
    pub name: String,
} */

#[derive(Component)]
pub struct Player {}

pub fn create_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/PNG/Soldier 1/soldier1_gun.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
