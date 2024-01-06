use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.add(SpawnAsteroidCommand);
}

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        // .add_state::<GameState>()
        .add_systems(Startup, setup)
        // .add_plugins((
        //     AsteroidDragPlugin,
        //     InputPlugin,
        //     PhysicsPlugin,
        //     PlanetEditorPlugin,
        //     UiPlugin,
        // ))
        .run();
}
