//! Startup system to set up the scene.
use crate::component;
use crate::resource;
use bevy::prelude::*;

pub fn generate_grid(
    size: i32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    plane_material: &Handle<StandardMaterial>,
    plane_material2: &Handle<StandardMaterial>,
) {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut counter = 0;
    for y in -size..size {
        for x in -size..size {
            if (x + y) % 2 == 0 {
                let x = x as f32;
                let y = y as f32;
                let vertices = [
                    ([x, 0.0, y], [0.0, 1.0, 0.0], [0.0, 0.0]),
                    ([x + 1.0, 0.0, y], [0.0, 1.0, 0.0], [1.0, 0.0]),
                    ([x + 1.0, 0.0, y + 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
                    ([x, 0.0, y + 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ];
                indices.push(counter * 4 + 1);
                indices.push(counter * 4 + 0);
                indices.push(counter * 4 + 3);
                indices.push(counter * 4 + 1);
                indices.push(counter * 4 + 3);
                indices.push(counter * 4 + 2);
                counter += 1;
                for (position, normal, uv) in vertices.iter() {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }
            }
        }
    }
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: plane_material.clone(),
        ..Default::default()
    });

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut counter = 0;
    for y in -size..size {
        for x in -size..size {
            if (x + y) % 2 != 0 {
                let x = x as f32;
                let y = y as f32;
                let vertices = [
                    ([x, 0.0, y], [0.0, 1.0, 0.0], [0.0, 0.0]),
                    ([x + 1.0, 0.0, y], [0.0, 1.0, 0.0], [1.0, 0.0]),
                    ([x + 1.0, 0.0, y + 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
                    ([x, 0.0, y + 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ];
                indices.push(counter * 4 + 1);
                indices.push(counter * 4 + 0);
                indices.push(counter * 4 + 3);
                indices.push(counter * 4 + 1);
                indices.push(counter * 4 + 3);
                indices.push(counter * 4 + 2);
                counter += 1;
                for (position, normal, uv) in vertices.iter() {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }
            }
        }
    }
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: plane_material2.clone(),
        ..Default::default()
    });
}

/// Startup system to set up the scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());
    let plane_material2 = materials.add(Color::rgb(0.5, 0.3, 0.3).into());

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    generate_grid(
        200,
        &mut commands,
        &mut meshes,
        &plane_material,
        &plane_material2,
    );

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_mesh.clone(),
            material: cube_material.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default());
    // light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(component::CameraLight);
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default());
    commands.insert_resource(resource::Data {
        cube_mesh,
        cube_material,
    });
}
