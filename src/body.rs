use crate::vec2d::Vec2D;

pub struct Body {
    mass: i32,
    position: Vec2D,
    velocity: Vec2D,
    acceleration: Vec2D,
}