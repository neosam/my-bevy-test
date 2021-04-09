#![no_std]

use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, render};

struct CameraLight;

struct Data {
    pub plane_mesh: Handle<Mesh>,
    pub plane_material: Handle<StandardMaterial>,

    pub cube_mesh: Handle<Mesh>,
    pub cube_material: Handle<StandardMaterial>,
}

fn setup(
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
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    }).insert(CameraLight);
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.insert_resource(Data {
        plane_mesh, plane_material, cube_mesh, cube_material,
    });
}
struct CameraPosition(f32, f32, f32);

fn camera_positioning(
        cam_pos: Res<CameraPosition>, 
        mut query: QuerySet<(
            Query<&mut Transform, With<render::camera::Camera>>,
            Query<&mut Transform, With<CameraLight>>
        )>,
    ) {
    let CameraPosition(x, y, distance) = *cam_pos;
    for mut transform in query.q0_mut().iter_mut() {
        *transform = Transform::from_xyz(x - distance, distance, y + distance)
                        .looking_at(Vec3::new(x, 0.0, y), Vec3::Y);
    }
    for mut transform in query.q1_mut().iter_mut() {
        *transform = Transform::from_xyz(x - 2.0, 10.0, y - 2.0);
    }
    
}

#[derive(Default)]
struct InputCommands {
    pub move_camera_right: bool,
    pub move_camera_left: bool,
    pub move_camera_bottom: bool,
    pub move_camera_up: bool,
}

enum InputEvent {
    SpawnCube,
}

fn user_input(
        mut input_commands: ResMut<InputCommands>,
        mut input_events: EventWriter<InputEvent>,
        key_codes: Res<Input<KeyCode>>
    ) {
    if key_codes.just_pressed(KeyCode::Right) {
        input_commands.move_camera_right = true;
    }
    if key_codes.just_pressed(KeyCode::Left) {
        input_commands.move_camera_left = true;
    }
    if key_codes.just_pressed(KeyCode::Down) {
        input_commands.move_camera_bottom = true;
    }
    if key_codes.just_pressed(KeyCode::Up) {
        input_commands.move_camera_up = true;
    }
    if key_codes.just_released(KeyCode::Right) {
        input_commands.move_camera_right = false;
    }
    if key_codes.just_released(KeyCode::Left) {
        input_commands.move_camera_left = false;
    }
    if key_codes.just_released(KeyCode::Down) {
        input_commands.move_camera_bottom = false;
    }
    if key_codes.just_released(KeyCode::Up) {
        input_commands.move_camera_up = false;
    }

    if key_codes.just_pressed(KeyCode::Space) {
        input_events.send(InputEvent::SpawnCube);
    }
}

fn camera_movement(mut cam_pos: ResMut<CameraPosition>, input_commands: Res<InputCommands>, time: Res<Time>) {
    if input_commands.move_camera_right {
        cam_pos.0 += time.delta_seconds() * 10.0;
        cam_pos.1 += time.delta_seconds() * 10.0;
    }
    if input_commands.move_camera_left {
        cam_pos.0 += time.delta_seconds() * -10.0;
        cam_pos.1 += time.delta_seconds() * -10.0;
    }
    if input_commands.move_camera_bottom {
        cam_pos.0 += time.delta_seconds() * -10.0;
        cam_pos.1 += time.delta_seconds() * 10.0;
    }
    if input_commands.move_camera_up {
        cam_pos.0 += time.delta_seconds() * 10.0;
        cam_pos.1 += time.delta_seconds() * -10.0;
    }
}

fn cube_spawner(
        mut commands: Commands,
        mut input_events: EventReader<InputEvent>,
        cam_pos: Res<CameraPosition>,
        data: Res<Data>,
    ) {
    for event in input_events.iter() {
        match event {
            &InputEvent::SpawnCube => {
                let CameraPosition(x, y, _) = *cam_pos;
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

fn main() {
    let mut builder = App::build();
    builder
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(CameraPosition(0.0, 0.0, 10.0))
        .insert_resource(InputCommands::default())
        .add_event::<InputEvent>()
        .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    builder.
        add_plugin(bevy_webgl2::WebGL2Plugin);
    builder
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(user_input.system())
        .add_system(camera_movement.system())
        .add_system(camera_positioning.system())
        .add_system(cube_spawner.system())
        .run();
}
