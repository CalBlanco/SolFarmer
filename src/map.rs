use bevy::prelude::*;

use super::{RESOLUTION_X, RESOLUTION_Y};

#[derive(Component)]
pub enum TileState {
    Immutable,
    Untoiled,
    Toiled,
    Planted,
}

impl TileState {
    pub fn value(&self) -> String {
        match self {
            TileState::Immutable => String::from("Immutable"),
            TileState::Untoiled => String::from("Untoiled"),
            TileState::Toiled => String::from("Toiled"),
            TileState::Planted => String::from("Planted"),
        }
    }
}

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Bundle)]
pub struct TileBundle {
    pub state: TileState,
    pub sprite: SpriteBundle,
    pub position: Position,
}

impl TileBundle {
    pub fn new (texture: Handle<Image>, x: i32, y: i32, state: TileState) -> TileBundle {
        // Get the tile coords
        let (x, y) = get_world(x, y);
        // Return the tile bundle
        TileBundle {
            sprite: SpriteBundle {
                texture:texture,
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            },
            state: state,
            position: Position(Vec2::new(x, y))
        }
    }
}


/// Returns true if point_a and point_b are within a range of dist
fn within_circle (point_a: (i32, i32), point_b: (i32, i32), dist: f32) -> bool {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;
    
    dist > (((a_x - b_x).pow(2) as f32) + ((a_y - b_y).pow(2) as f32)).sqrt()
}

pub fn distance_int_from_point (point_a: (i32, i32), point_b: (i32, i32)) -> f32 {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;

    // Return distance, clamped, as an integer
    (((a_x - b_x).pow(2) as f32) + ((a_y - b_y).pow(2) as f32)).sqrt()
}

/// Returns true if point is within rect made by bottom_left and top_right
fn within_rect(bottom_left: (i32, i32), top_right: (i32, i32), point: (i32, i32)) -> bool {
    let (btm_x, btm_y) = bottom_left;
    let (top_x, top_y) = top_right;
    let (pt_x, pt_y) = point;

    // Return if we're within the rectangle
    (pt_x >= btm_x && pt_x <= top_x) && (pt_y >= btm_y && pt_y <= top_y)
}

pub fn draw_background (mut commands: Commands, assets: Res<AssetServer>) {
    let num_x = (RESOLUTION_X / 32.) as i32;
    let num_y = (RESOLUTION_Y / 32.) as i32;

    let middle_tile = (num_x / 2, num_y / 2);
    let rock_pos_1 = (0, 0);
    let rock_pos_2 = (0, num_y - 1);
    let rock_pos_3 = (num_x - 1, 0);
    let rock_pos_4 = (num_x - 1, num_y - 1);

    // Load the background
    for x in 0..num_x {
        for y in 0..num_y {
            // Draw the Corners of the map
            if within_circle(rock_pos_1, (x, y), 5.) || within_circle(rock_pos_2, (x, y), 5.) || within_circle(rock_pos_3, (x, y), 5.) || within_circle(rock_pos_4, (x, y), 5.) {
                commands.spawn(TileBundle::new(
                    assets.load("tiles/wood_ruined.png"),
                    x,
                    y,
                    TileState::Immutable
                ));
            }
            // Draw the Middle Area (Ready Planting Zone)            
            else if within_circle(middle_tile, (x, y), 4.) {
                commands.spawn(TileBundle::new(
                    assets.load("tiles/farmtile.png"),
                    x,
                    y,
                    TileState::Toiled   
                ));
            }
            // Draw the "house" at the bottom of the map
            else if within_rect((6, 0), (33, 3), (x, y)) {
                commands.spawn(TileBundle::new(
                    assets.load("tiles/concrete.png"),
                    x,
                    y,
                    TileState::Immutable
                ));
            } 
            // Place Torches on Concrete tiles
            else if (x == 7 || x == 32) && (y == 6 || y == 15) {
                commands.spawn(TileBundle::new(
                    assets.load("tiles/concrete.png"),
                    x,
                    y,
                    TileState::Immutable
                ));
                commands.spawn( SpriteBundle {
                    texture: assets.load("images/torch.png"),
                    transform: Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 1.),
                    ..default()
                });
            } 
            // Draw the basic ground behind everything
            else {
                commands.spawn(TileBundle::new(
                    assets.load("tiles/redgrass.png"),
                    x,
                    y,
                    TileState::Untoiled
                )); 
            }
        }
    }
}


pub fn get_tile(x:f32, y:f32) -> (i32, i32) {
    let x = x + 16.;
    let y = y + 16.;
    ((x / 32.) as i32, (y/ 32.) as i32)
}

pub fn get_world(x:i32, y:i32) -> (f32, f32) {
   ( (x * 32 ) as f32, (y*32) as f32)
}
