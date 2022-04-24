use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod snake;
mod windowsize;

use snake::*;
use windowsize::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake O_o".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(
            Color::rgb(0.0, 0.0, 0.0)
        ))
        .insert_resource(BodysSnake::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(camera)
        .add_startup_system(spawn_snake)
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_system(snake_movement_input.before(move_snake))
        .add_system(game_over.after(move_snake))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(move_snake)
                .with_system(snake_eating.after(move_snake))
                .with_system(snake_growth.after(snake_eating))
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling)
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}