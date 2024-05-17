use bevy::prelude::*;

use super::{RESOLUTION_X, RESOLUTION_Y};

// Returns true if point_a and point_b are within a range of dist
fn within_circle (point_a: (i32, i32), point_b: (i32, i32), dist: f32) -> bool {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;
    
    dist > (((a_x - b_x).abs().pow(2) as f32) + ((a_y - b_y).abs().pow(2) as f32)).sqrt()
}

// Returns true if point is within rect made by bottom_left and top_right
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
    let rock_pos_2 = (0, num_y);
    let rock_pos_3 = (num_x, 0);
    let rock_pos_4 = (num_x, num_y);

    let plot_bottom_left = (12, 7);
    let plot_top_right = (28, 15);

    // Load the background
    for x in 0..num_x {
        for y in 0..num_y {
            // Draw the basic ground behind everything
            commands.spawn(
                SpriteBundle {
                    texture: assets.load("tiles/redgrass.png"),
                    transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, -1.),
                    ..default()
                }
            );
            // Draw the "Rocks"
            if within_circle(rock_pos_1, (x, y), 4.) || within_circle(rock_pos_2, (x, y), 4.) || within_circle(rock_pos_3, (x, y), 4.) || within_circle(rock_pos_4, (x, y), 4.) {
                commands.spawn(
                    SpriteBundle {
                        texture: assets.load("tiles/wood_ruined.png"),
                        transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, 0.),
                        ..default()
                    }
                );
            }
            // Draw the Middle Area (Planting Zone)            
            if within_circle(middle_tile, (x, y), 4.) {
                commands.spawn(
                    SpriteBundle {
                        texture: assets.load("tiles/farmtile.png"),
                        transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, 0.),
                        ..default()
                    }
                );
            }
            // Draw the "house" at the bottom of the map
            if within_rect((5, 0), (35, 3), (x, y)) {
                commands.spawn(
                    SpriteBundle {
                        texture: assets.load("tiles/concrete.png"),
                        transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, 0.),
                        ..default()
                    }
                );
            }
        }
    }
}