use std::time::Duration;

use bevy::prelude::*;


// Animate a player (or enemy)
// four states (face left, face right, face down, face up)

#[derive(Component)]
pub enum HumanAnimState {
    FaceDown,
    FaceRight,
    FaceUp,
    FaceLeft
}


impl HumanAnimState {
    pub fn getRect(&self) -> Rect {
        match self {
            HumanAnimState::FaceDown => {Rect{ min: Vec2{ x: 0., y: 0.}, max: Vec2{ x: 31., y: 31.}}},
            HumanAnimState::FaceRight => {Rect{ min: Vec2{ x: 32., y: 0.}, max: Vec2{ x: 63., y: 31.}}},
            HumanAnimState::FaceUp => {Rect{ min: Vec2{ x: 64., y: 0.}, max: Vec2{ x: 95., y: 31.}}},
            HumanAnimState::FaceLeft => {Rect{ min: Vec2{ x: 96., y: 0.}, max: Vec2{ x: 127., y: 31.}}},
        } 
    }
}

#[derive(Component)]
pub struct HumanStateController {
    state: HumanAnimState
}



#[derive(Bundle)]
pub struct HumanAnimator {
    sprite_bundle: SpriteBundle,
    state: HumanAnimState,
}

impl HumanAnimator {
    pub fn new(image: Handle<Image>, state: HumanAnimState, t_off: Vec3) -> HumanAnimator {
        HumanAnimator {
            sprite_bundle: SpriteBundle {
                texture: image,
                sprite: Sprite {
                    rect: Some(state.getRect()),
                    ..default()
                },
                transform: Transform::from_xyz(t_off.x, t_off.y, t_off.z),
                ..default()
            },
            state: state
        }
    }
}


