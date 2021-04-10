//! Evaluate use input and set InputCommands and triggers InputEvents.
use crate::event;
use crate::resource;
use bevy::prelude::*;

/// Evaluate use input and set InputCommands and triggers InputEvents.
pub fn user_input(
    mut input_commands: ResMut<resource::InputCommands>,
    mut input_events: EventWriter<event::InputEvent>,
    key_codes: Res<Input<KeyCode>>,
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

    if key_codes.just_pressed(KeyCode::W) {
        input_commands.entity_move_up = true;
    }
    if key_codes.just_pressed(KeyCode::S) {
        input_commands.entity_move_down = true;
    }
    if key_codes.just_pressed(KeyCode::A) {
        input_commands.entity_move_left = true;
    }
    if key_codes.just_pressed(KeyCode::D) {
        input_commands.entity_move_right = true;
    }
    if key_codes.just_released(KeyCode::W) {
        input_commands.entity_move_up = false;
    }
    if key_codes.just_released(KeyCode::S) {
        input_commands.entity_move_down = false;
    }
    if key_codes.just_released(KeyCode::A) {
        input_commands.entity_move_left = false;
    }
    if key_codes.just_released(KeyCode::D) {
        input_commands.entity_move_right = false;
    }

    if key_codes.just_pressed(KeyCode::Space) {
        input_events.send(event::InputEvent::SpawnCube);
    }
}
