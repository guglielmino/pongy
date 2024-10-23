use rand::Rng;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

pub fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(700., 500.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 150.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 150.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    ));
}

pub fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 100. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-250. + 75., 250. - 75.);
        } else if input.pressed(settings.move_down) {
            pos.translation.y -= 100. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-250. + 75., 250. - 75.);
        }
    }
}

#[derive(Component)]
pub struct Ball(Vec2);


pub fn spawn_ball(mut commands: Commands) {
    commands.spawn((SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball(Vec2::new(-100., 0. ))));
}

pub fn move_ball(
   mut  balls: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    for(mut pos, ball) in &mut balls {
        pos.translation += ball.0.extend(0.) * time.delta_seconds();
    }
}


const BALLWIDTH: f32 = 25.0;
const PWIDTH: f32 = 10.;
const PHEIGHT: f32 = 150.;

pub fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    
    for (ball, mut velocity) in &mut balls {
        if ball.translation.y.abs() + BALLWIDTH/ 2. > 250. {
            velocity.0.y *= -1.;
        }

            for paddle in &paddles {
            if 
            ball.translation.x - BALLWIDTH / 2. < paddle.translation.x + PWIDTH / 2. &&
            ball.translation.y - BALLWIDTH / 2. < paddle.translation.y + PHEIGHT / 2. &&
            ball.translation.x + BALLWIDTH / 2. > paddle.translation.x - PWIDTH / 2. &&
            ball.translation.y + BALLWIDTH / 2. > paddle.translation.y - PHEIGHT / 2. {
                velocity.0 *= -1.;
                velocity.0.y = rand::thread_rng().gen::<f32>() * 100.;
            }
        }
    }
}