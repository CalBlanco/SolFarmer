use bevy::prelude::*;

use super::{RESOLUTION_X, RESOLUTION_Y};

pub fn point_dist (point_a: (i32, i32), point_b: (i32, i32)) -> f32 {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;
    
    (((a_x - b_x).abs().pow(2) as f32) + ((a_y - b_y).abs().pow(2) as f32)).sqrt()
}

pub fn draw_background (mut commands: Commands, assets: Res<AssetServer>) {
    let num_x = (RESOLUTION_X / 32.) as i32;
    let num_y = (RESOLUTION_Y / 32.) as i32;

    let middle_tile = (num_x / 2, num_y / 2);
    let rock_pos_1 = (0, 0);
    let rock_pos_2 = (0, num_y);
    let rock_pos_3 = (num_x, 0);
    let rock_pos_4 = (num_x, num_y);

    // Load the background
    for x in 0..num_x {
        for y in 0..num_y {
            
            // Draw the "Rocks"
            if (point_dist(rock_pos_1, (x, y)) < 4.) || (point_dist(rock_pos_2, (x, y)) < 4.) || (point_dist(rock_pos_3, (x, y)) < 4.) || (point_dist(rock_pos_4, (x, y)) < 4.) {

            }
            // Draw the Middle Area (Planting Zone)            
            else if point_dist(middle_tile, (x, y)) < 4. {
                
            }
            // Draw the "ground"
            else {
                commands.spawn(
                    SpriteBundle {
                        texture: assets.load("tiles/redgrass.png"),
                        transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, -1.),
                        ..default()
                    }
                );
            }

        }
    }
}