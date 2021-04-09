//! Spawns cubes on CameraPosition if the is a InputEvent::SpawnCube.
use crate::event;
use crate::resource;
use bevy::prelude::*;

/// Spawns cubes on CameraPosition if the is a InputEvent::SpawnCube.
pub fn cube_spawner(
    mut commands: Commands,
    mut input_events: EventReader<event::InputEvent>,
    cam_pos: Res<resource::CameraPosition>,
    data: Res<resource::Data>,
) {
    for event in input_events.iter() {
        match event {
            &event::InputEvent::SpawnCube => {
                let resource::CameraPosition(x, y, _) = *cam_pos;
                commands.spawn_bundle(PbrBundle {
                    mesh: data.cube_mesh.clone(),
                    material: data.cube_material.clone(),
                    transform: Transform::from_xyz(x, 0.5, y),
                    ..Default::default()
                });
            }
        }
    }
}
