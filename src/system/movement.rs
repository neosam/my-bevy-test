//! System which moves selected entities.
use crate::component;
use crate::resource;
use bevy::prelude::*;

/// System which moves selected entities.
pub fn movement(
    mut query: Query<&mut Transform, With<component::Selected>>,
    input_commands: Res<resource::InputCommands>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        if input_commands.entity_move_up {
            *transform = Transform::from_xyz(
                transform.translation.x + 10.0 * time.delta_seconds(),
                transform.translation.y,
                transform.translation.z - 10.0 * time.delta_seconds(),
            );
        }
        if input_commands.entity_move_down {
            *transform = Transform::from_xyz(
                transform.translation.x - 10.0 * time.delta_seconds(),
                transform.translation.y,
                transform.translation.z + 10.0 * time.delta_seconds(),
            );
        }
        if input_commands.entity_move_left {
            *transform = Transform::from_xyz(
                transform.translation.x - 10.0 * time.delta_seconds(),
                transform.translation.y,
                transform.translation.z - 10.0 * time.delta_seconds(),
            );
        }
        if input_commands.entity_move_right {
            *transform = Transform::from_xyz(
                transform.translation.x + 10.0 * time.delta_seconds(),
                transform.translation.y,
                transform.translation.z + 10.0 * time.delta_seconds(),
            );
        }
    }
}
