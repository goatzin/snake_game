// Here is the snake system and the food system :)
use crate::windowsize;

use bevy::prelude::*;
use rand::prelude::random;
use windowsize::*;

const WINDOW_WIDTH:u32 = 10;
const WINDOW_HEIGHT:u32 = 10;

const SNAKE_COLOR:Color = Color::rgb(255.0, 180.0, 0.0);
const FOOD_COLOR:Color = Color::rgb(66.0, 135.0, 245.0);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Down,
    Up
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Up => Self::Down
        }
    }
}

pub struct GrowthEvent;
pub struct GameOverEvent;

#[derive(Component)]
pub struct Snake {
    direction:Direction
}
#[derive(Component)]
pub struct BodySnake;
#[derive(Default)]
pub struct BodysSnake(Vec<Entity>);
#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(Food)
    .insert(Position {
        x: (random::<f32>() * WINDOW_WIDTH as f32) as i32,
        y: (random::<f32>() * WINDOW_HEIGHT as f32) as i32
    })
    .insert(WindowSize::square(0.8));
}

// Snake
pub fn spawn_snake(mut commands: Commands, mut body: ResMut<BodysSnake>) {
    *body = BodysSnake(vec![
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        })
            .insert(Snake{ direction:Direction::Up })
            .insert(BodySnake)
            .insert(Position { x: 3, y: 3 })
            .insert(WindowSize::square(0.8))
            .id(),
        spawn_body(commands, Position{ x: 3, y: 3 }),
    ])
}
pub fn move_snake(
    body:ResMut<BodysSnake>,
    mut heads:Query<(Entity, &Snake)>,
    mut last_tail_pos:ResMut<LastTailPosition>,
    mut position:Query<&mut Position>,
    mut game_over:EventWriter<GameOverEvent>
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let body_position = body.0
            .iter()
            .map(|e| *position.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        *last_tail_pos = LastTailPosition(Some(*body_position.last().unwrap()));
        let mut head_pos = position.get_mut(head_entity).unwrap(); 
        match &head.direction {
            Direction::Right => {
                head_pos.x += 1;
            },
            Direction::Left => {
                head_pos.x -= 1;
            },
            Direction::Down => {
                head_pos.y -= 1;
            },
            Direction::Up => {
                head_pos.y += 1;
            }
        };
        if head_pos.x < 0 || head_pos.y < 0
            || head_pos.x as u32 >= WINDOW_WIDTH || head_pos.y as u32 >= WINDOW_HEIGHT 
        {
            game_over.send(GameOverEvent);
        }
        if body_position.contains(&head_pos) {
            game_over.send(GameOverEvent);
        }
        body_position.iter()
            .zip(body.0.iter().skip(1))
            .for_each(|(pos, body)| {
                *position.get_mut(*body).unwrap() = *pos;
            })
    }
}
pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads:Query<&mut Snake>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir:Direction = if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn spawn_body(mut commands:Commands, position:Position) -> Entity {
    commands.spawn_bundle(SpriteBundle {
        sprite:Sprite {
            color: SNAKE_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(BodySnake)
    .insert(position)
    .insert(WindowSize::square(0.8))
    .id()
}
pub fn snake_eating(
    mut commands:Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_position:Query<(Entity, &Position), With<Food>>,
    snake_position:Query<&Position, With<Snake>>
) {
    for snake_pos in snake_position.iter() {
        for (ent, food_pos) in food_position.iter() {
            if food_pos == snake_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}
pub fn snake_growth(
    commands:Commands,
    last_tail_pos:Res<LastTailPosition>,
    mut body:ResMut<BodysSnake>,
    mut growth_reader:EventReader<GrowthEvent>
) {
    if growth_reader.iter().next().is_some() {
        body.0.push(spawn_body(commands, last_tail_pos.0.unwrap()));
    }
}

pub fn game_over(
    mut commands:Commands,
    mut reader:EventReader<GameOverEvent>,
    body_res:ResMut<BodysSnake>,
    food:Query<Entity, With<BodySnake>>,
    body:Query<Entity, With<BodySnake>>
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(body.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, body_res);
    }
}