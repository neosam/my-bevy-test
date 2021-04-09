use crate::component;
use crate::resource;
use bevy::{prelude::*, render};

pub fn camera_positioning(
    cam_pos: Res<resource::CameraPosition>,
    mut query: QuerySet<(
        Query<&mut Transform, With<render::camera::Camera>>,
        Query<&mut Transform, With<component::CameraLight>>,
    )>,
) {
    let resource::CameraPosition(x, y, distance) = *cam_pos;
    for mut transform in query.q0_mut().iter_mut() {
        *transform = Transform::from_xyz(x - distance, distance, y + distance)
            .looking_at(Vec3::new(x, 0.0, y), Vec3::Y);
    }
    for mut transform in query.q1_mut().iter_mut() {
        *transform = Transform::from_xyz(x - 2.0, 10.0, y - 2.0);
    }
}
