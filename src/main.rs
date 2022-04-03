mod utils;
use crate::utils::rotate_about;
use bevy::prelude::*;
use bevy_cable::{Cable, CableHead, CableNode, CablePlugin, Constraint, Velocity};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CablePlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Crown;

#[derive(Component)]
struct Arm;

const SPRITE_SIZE: f32 = 75.0;
const ARM_WIDTH: f32 = 15.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let arm_colors = vec![
        Color::LIME_GREEN,
        Color::RED,
        Color::BLUE,
        Color::YELLOW,
        Color::PINK,
        Color::PURPLE,
        Color::ORANGE,
        Color::SILVER,
    ];

    let crown = commands
        .spawn()
        .insert(Crown)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * SPRITE_SIZE),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let children: Vec<Entity> = (0..8)
        .map(|i| {
            let transform = Transform::from_translation(rotate_about(
                Vec3::ZERO,
                Vec3::new(SPRITE_SIZE / 2.0, SPRITE_SIZE / 2.0, 0.0),
                f32::to_radians(45.0 * i as f32),
            ));
            let head = commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: arm_colors[i],
                        custom_size: Some(Vec2::ONE * ARM_WIDTH),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert_bundle((
                    Arm,
                    CableHead,
                    Cable,
                    Velocity {
                        translation: Vec3::ZERO,
                    },
                ))
                .id();
            commands.entity(head).with_children(|parent| {
                let mut prev_id = head;
                for j in 0..5 {
                    let child_transform =
                        Transform::from_translation(transform.translation * j as f32);
                    let id = parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::ONE * ARM_WIDTH),
                                color: arm_colors[i],
                                ..Default::default()
                            },
                            transform: child_transform,
                            ..Default::default()
                        })
                        .insert_bundle((
                            CableNode {
                                previous_position: child_transform.translation,
                            },
                            Cable,
                            Velocity {
                                translation: child_transform.translation.normalize() * 20.0,
                            },
                        ))
                        .id();
                    parent.spawn().insert(Constraint {
                        node_1: prev_id,
                        node_2: id,
                        desired_distance: ARM_WIDTH,
                    });
                    prev_id = id;
                }
            });
            head
        })
        .collect();
    commands.entity(crown).push_children(&children);
}
