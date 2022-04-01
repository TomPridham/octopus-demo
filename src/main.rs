mod utils;
use crate::utils::rotate_about;
use bevy::prelude::*;
use bevy_cable::{Cable, CableNode, CablePlugin, Constraint};

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
                Vec3::new(45.0, 45.0, 0.0),
                f32::to_radians(45.0 * i as f32),
            ));
            let head = commands
                .spawn()
                .insert(Arm)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: arm_colors[i],
                        custom_size: Some(Vec2::ONE * ARM_WIDTH),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .id();
            commands.entity(head).with_children(|parent| {
                let mut prev_id = head;
                for i in 1..5 {
                    let child_transform =
                        Transform::from_translation(transform.translation * i as f32);
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
                        ))
                        .id();
                    parent.spawn().insert(Constraint {
                        node_1: prev_id,
                        node_2: id,
                        desired_distance: SPRITE_SIZE,
                    });
                    prev_id = id;
                }
            });
            head
        })
        .collect();
    commands.entity(crown).push_children(&children);
}
