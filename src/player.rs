use bevy::prelude::*;
use rand::Rng;

use crate::{game, map::{self, TileBundle, TileState}, spriteanims};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mouse;

const SPAWN_X: i32 = 20;
const SPAWN_Y: i32 = 20;
const MOVE_SPEED: f32 = 90.;

#[derive(Component)]
pub struct PlayerAttack (pub Timer);

const ATTACK_COOLDOWN: f32 = 0.4;

#[derive(Component)]
pub struct Hoe;

#[derive(Component, Clone)]
pub enum PlayerTool {
    Tiller,
    Planter,
    Rake
}


#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_anim: spriteanims::HumanAnimator,
    ply:  Player,
    attack: PlayerAttack,
    tool: PlayerTool
}

impl PlayerBundle {
    pub fn new(anim: spriteanims::HumanAnimator) -> PlayerBundle {
        PlayerBundle {
            sprite_anim: anim,
            ply: Player,
            attack: PlayerAttack(Timer::from_seconds(ATTACK_COOLDOWN, TimerMode::Once)),
            tool: PlayerTool::Planter
        }
    }
}   



pub fn setup(mut commands: Commands, assets: Res<AssetServer>){
    let spawn = map::get_world(SPAWN_X, SPAWN_Y);

    commands.spawn( 
        PlayerBundle::new(spriteanims::HumanAnimator::new(
            assets.load("entity/human_profile/ed_sheet.png"), spriteanims::HumanAnimState::FaceDown, Vec3 {x: spawn.0, y: spawn.1, z :1.0}))
    ).with_children(|parent| {
        parent.spawn((SpriteBundle {
            texture: assets.load("images/hoe1.png"),
            transform: Transform::from_xyz(-8., 8., 5.),
            ..default()
        },
        Hoe,
    ));
    });

    commands.spawn( (
        SpriteBundle {
            texture: assets.load("tiles/highlight.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Mouse
    ));
}

/// Move the player around 
pub fn player_input(
    mut query: Query<(Entity, &mut Transform, &mut PlayerAttack, &mut Sprite, &mut spriteanims::HumanAnimState, &mut PlayerTool), With<Player>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mouse: Res<game::MyWorldCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>
)
{
    if let Ok((_e, mut transform, mut player_attack, mut sprite, mut state, mut tool )) = query.get_single_mut() {
        // Tick the attack timer
        player_attack.0.tick(time.delta());
        let move_distance = MOVE_SPEED * time.delta_seconds();
        sprite.rect = Some(state.getRect());

        if keycode.pressed(KeyCode::KeyW) {
            
            *state = spriteanims::HumanAnimState::FaceUp;
            transform.translation.y += move_distance;
        }
        if keycode.pressed(KeyCode::KeyS) {
            *state = spriteanims::HumanAnimState::FaceDown;
            transform.translation.y -= move_distance;
        }
        if keycode.pressed(KeyCode::KeyD) {
            *state = spriteanims::HumanAnimState::FaceRight;
            transform.translation.x += move_distance;
        }
        if keycode.pressed(KeyCode::KeyA) {
            *state = spriteanims::HumanAnimState::FaceLeft;
            transform.translation.x -= move_distance;
        }


        if keycode.just_pressed(KeyCode::Digit1){
            *tool = PlayerTool::Tiller;
        }
        if keycode.just_pressed(KeyCode::Digit2){
            *tool = PlayerTool::Planter;
        }
        if keycode.just_pressed(KeyCode::Digit3){
            *tool = PlayerTool::Rake;
        }

        // If the player can attack and is trying to attack
        if (keycode.pressed(KeyCode::Space)) && (player_attack.0.finished()) {
            // Reset the attack timer
            player_attack.0.reset();
        }

        // Handle the placing/removal of the plant units
        if mouse_input.just_pressed(MouseButton::Left) {
            // Get the indices of the tile clicked on
            let (x, y) = map::get_tile(mouse.0.x, mouse.0.y);
            println!("{}, {}", x, y);
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            
        }
    } 
}

pub fn render_tile_highlight(
    mouse: Res<game::MyWorldCoords>,
    mut tile_highlight: Query<&mut Transform, With<Mouse>>
)
{
    if let Ok(mut transform) = tile_highlight.get_single_mut() {
        let (x, y) = map::get_tile(mouse.0.x, mouse.0.y);
        let (x, y) = map::get_world(x, y);

        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn hoe_swing(
    player_query: Query<&PlayerAttack, With<Player>>,
    mut hoe_query: Query<&mut Transform, With<Hoe>>,
) {
    if let Ok(player_attack) = player_query.get_single() {
        // Are we attacking currently?
        if !player_attack.0.finished() {
            for mut hoe_transform in hoe_query.iter_mut() {
                // Calculate the progress of the swing
                let elapsed = player_attack.0.elapsed().as_secs_f32();
                let duration = player_attack.0.duration().as_secs_f32();
                let swing_progress = elapsed / duration;
                // Use the progress of the swing to determine the angle and the offset
                let angle = (swing_progress * 360.).to_radians();
                let offset_x = 16. * (angle + std::f32::consts::PI).cos();
                let offset_y = 16. * (angle + std::f32::consts::PI).sin();
                // Move the hoe in a circle around the player
                hoe_transform.translation.x = offset_x;
                hoe_transform.translation.y = offset_y;
                // Rotate the hoe while moving it to make it look like a full swing
                hoe_transform.rotation = Quat::from_rotation_z(angle + (0.5 * std::f32::consts::PI));
            }
        } else {
            for mut hoe_transform in hoe_query.iter_mut() {
                // Calculate the offsets using an angle of 0 degrees
                let angle = 0 as f32;
                let offset_x = 16. * (angle + std::f32::consts::PI).cos();
                let offset_y = 16. * (angle + std::f32::consts::PI).sin();
                // Keep the hoe at default position
                hoe_transform.translation.x = offset_x;
                hoe_transform.translation.y = offset_y;
                // Keep the hoe at default rotation
                hoe_transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}


#[derive(Event)]
pub struct ClickEvent((i32, i32), PlayerTool);

/// Control tile placement
pub fn mouse_tile_select(
    player_query: Query<&PlayerTool, With<Player>>,
    mouse: Res<game::MyWorldCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut click_event: EventWriter<ClickEvent>,
){
    let Ok((tool)) = player_query.get_single() else {warn!("No tool"); return;};
    if mouse_input.just_pressed(MouseButton::Left) {
       // println!("Clicked Something {}", tile_query.iter().len());
        let mouse_tile = map::get_tile(mouse.0.x, mouse.0.y);
        click_event.send(ClickEvent((mouse_tile.0, mouse_tile.1), tool.clone()));
    }
    
}

pub fn react_to_mouse_event(
    mut ev_levelup: EventReader<ClickEvent>,
    tile_query: Query<(Entity, &mut map::TileState, &map::Position, &mut Sprite)>,
    assets: Res<AssetServer>,
    mut commands: Commands,
){
    {
        for ev in ev_levelup.read() {
            for (e, s, p, _) in &tile_query{
                let tile_tile = map::get_tile(p.0.x, p.0.y);
                
                if ev.0 == tile_tile {
                   // println!("Clicked Tile: ({},{}) has tile state: {}", mouse_tile.0, mouse_tile.1, s.value());
                    match ev.1 {
                        PlayerTool::Planter => {
                            match *s {
                                map::TileState::Toiled => {
                                    commands.entity(e).despawn();
                                    let rng = rand::thread_rng().gen_range(0..3);
                                    let str = match rng {
                                        0 => String::from("tiles/farmtile_seeds_green.png"),
                                        1 => String::from("tiles/farmtile_seeds_pink.png"),
                                        _ => String::from("tiles/farmtile_seeds_yellow.png"),
                                    };
                                    commands.spawn(
                                        map::TileBundle::new(assets.load(str), tile_tile.0, tile_tile.1, map::TileState::Planted)
                                    );
                                },
                                _ => {}
                            }
                        },
                        PlayerTool::Tiller => {
                            match *s {
                                map::TileState::Untoiled => {
                                    commands.entity(e).despawn();
                                    
                                    commands.spawn(
                                        map::TileBundle::new(assets.load("tiles/farmtile.png"), tile_tile.0, tile_tile.1, map::TileState::Toiled)
                                    );
                                },
                                _ => {}
                            }
                        },
                        PlayerTool::Rake => {
                            match *s {
                                map::TileState::Immutable => {
                                    
                                },
                                _ => {
                                    commands.entity(e).despawn();
                                    
                                    commands.spawn(
                                        map::TileBundle::new(assets.load("tiles/redgrass.png"), tile_tile.0, tile_tile.1, map::TileState::Untoiled)
                                    );
                                }
                            }
                        },
                    }
                    
                }
            }
        }
    }
}

/*
for (e, s, p, _) in &tile_query{
            let tile_tile = map::get_tile(p.0.x, p.0.y);
            
            if mouse_tile == tile_tile {
               // println!("Clicked Tile: ({},{}) has tile state: {}", mouse_tile.0, mouse_tile.1, s.value());
                match tool {
                    PlayerTool::Planter => {
                        match *s {
                            map::TileState::Toiled => {
                                commands.entity(e).despawn();
                                let rng = rand::thread_rng().gen_range(0..3);
                                let str = match rng {
                                    0 => String::from("tiles/farmtile_seeds_green.png"),
                                    1 => String::from("tiles/farmtile_seeds_pink.png"),
                                    _ => String::from("tiles/farmtile_seeds_yellow.png"),
                                };
                                commands.spawn(
                                    map::TileBundle::new(assets.load(str), tile_tile.0, tile_tile.1, map::TileState::Planted)
                                );
                            },
                            _ => {}
                        }
                    },
                    PlayerTool::Tiller => {
                        match *s {
                            map::TileState::Untoiled => {
                                commands.entity(e).despawn();
                                
                                commands.spawn(
                                    map::TileBundle::new(assets.load("tiles/farmtile.png"), tile_tile.0, tile_tile.1, map::TileState::Toiled)
                                );
                            },
                            _ => {}
                        }
                    },
                    PlayerTool::Rake => {
                        match *s {
                            map::TileState::Immutable => {
                                
                            },
                            _ => {
                                commands.entity(e).despawn();
                                
                                commands.spawn(
                                    map::TileBundle::new(assets.load("tiles/redgrass.png"), tile_tile.0, tile_tile.1, map::TileState::Untoiled)
                                );
                            }
                        }
                    },
                }
                
            }
        }
*/