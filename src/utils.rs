use bevy::prelude::*;

pub fn rotate_about(origin: Vec3, current_pos: Vec3, rotation_angle: f32) -> Vec3 {
    let new_origin = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(origin.x, origin.y, 1.0),
    );

    let rotate = Mat3::from_cols(
        Vec3::new(rotation_angle.cos(), rotation_angle.sin(), 0.0),
        Vec3::new(-rotation_angle.sin(), rotation_angle.cos(), 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let undo_origin = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-origin.x, -origin.y, 1.0),
    );

    new_origin * rotate * undo_origin * current_pos
}
