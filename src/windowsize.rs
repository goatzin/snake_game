use bevy::prelude::*;

const WINDOW_WIDTH:u32 = 10;
const WINDOW_HEIGHT:u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct WindowSize {
    width:f32,
    height:f32
}
impl WindowSize {
    pub fn square(x:f32) -> Self {
        Self {
            width: x,
            height: x
        }
    }
}

pub fn size_scaling(windows:Res<Windows>, mut q:Query<(&WindowSize, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / WINDOW_WIDTH as f32 * window.width() as f32,
            sprite_size.height / WINDOW_HEIGHT as f32 * window.height() as f32,
            1.0
        );
    }
}
pub fn position_translation(windows:Res<Windows>, mut q:Query<(&Position, &mut Transform)>) {
    fn convert(pos:f32, bound_window:f32, bound_game:f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, WINDOW_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, WINDOW_HEIGHT as f32),
            0.0
        );
    }
}