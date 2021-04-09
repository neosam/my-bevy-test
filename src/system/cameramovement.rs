//! Handles the camera movement based on InputCommand resource.
use crate::resource;
use bevy::prelude::*;

/// Handles the camera movement based on InputCommand resource.
pub fn camera_movement(
    mut cam_pos: ResMut<resource::CameraPosition>,
    input_commands: Res<resource::InputCommands>,
    time: Res<Time>,
) {
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
