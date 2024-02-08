use bevy::prelude::*;

#[derive(Component)]
struct Transform {
    position: Vec2,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
