//! Startup system to set up the scene.
use crate::component;
use crate::resource;
use bevy::prelude::*;

/// Startup system to set up the scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));
    let plane_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: plane_mesh.clone(),
        material: plane_material.clone(),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: cube_mesh.clone(),
        material: cube_material.clone(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(component::CameraLight);
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.insert_resource(resource::Data {
        plane_mesh,
        plane_material,
        cube_mesh,
        cube_material,
    });
}
