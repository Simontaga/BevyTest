use bevy::window::PrimaryWindow;
use bevy::{prelude::*, transform};

pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const BULLET_SPEED: f32 = 600.0;
pub const BULLET_MAX_TRAVEL: f32 = 350.0;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    direction: Direction,
}

#[derive(Component)]
struct Bullet {
    direction: Direction,
    animation_index: u8,
    travel_distance: f32
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_system(handle_player_movement)
        .add_system(update_bullets)
        .add_system(animate_bullet)
        .add_system(despawn_bullet)
        .run();
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player {
            direction: Direction::Up,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &mut Bullet)>, time: Res<Time>) {
    for (mut transform, mut bullet) in bullet_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        match bullet.direction {
            Direction::Up => {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            Direction::Down => {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
            Direction::Left => {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            Direction::Right => {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
        }
        let transformation = direction * BULLET_SPEED * time.delta_seconds();
        bullet.travel_distance += 1.0 + BULLET_SPEED * time.delta_seconds();
        transform.translation += transformation;
    }
}

fn handle_player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut pl_query: Query<&mut Transform, With<Player>>,
    mut pl_struct: Query<&mut Player>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(mut transform) = pl_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut pl_str = pl_struct.get_single_mut().unwrap();

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            pl_str.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            pl_str.direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            pl_str.direction = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            pl_str.direction = Direction::Down;
        }

        if keyboard_input.pressed(KeyCode::Space) {
            //println!("{:?}", pl_str.direction);
            shoot(
                commands,
                asset_server,
                transform.translation,
                pl_str.direction,
                texture_atlases,
            );
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: Vec3,
    direction: Direction,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /*
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            texture: asset_server.load("bullet.png"),
            ..default()
        },
        Bullet {
            direction
        },
    ));
     */

    let texture_handle = asset_server.load("bullet/bullet-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            ..default()
        },
        Bullet {
            direction,
            animation_index: 0,
            travel_distance: 0.0,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn despawn_bullet(
    mut commands: Commands,
    mut query: Query<(&Bullet, Entity, With<Bullet>)>,
) {
   for (bullet, entity, _) in &mut query {
    if bullet.travel_distance > BULLET_MAX_TRAVEL {
        commands.entity(entity).despawn();
    }
   }
}


fn animate_bullet(
    time: Res<Time>,
    mut query: Query<(&mut Bullet, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut bullet, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if bullet.animation_index == 2 {
                bullet.animation_index = 0;
            } else {
                bullet.animation_index += 1;
            }

            sprite.index = bullet.animation_index as usize;
            println!("{}", sprite.index)
        }
    }
}
