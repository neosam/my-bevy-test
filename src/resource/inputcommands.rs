//! Fields based on user input evaluation

/// Fields based on user input evaluation
#[derive(Default)]
pub struct InputCommands {
    pub move_camera_right: bool,
    pub move_camera_left: bool,
    pub move_camera_bottom: bool,
    pub move_camera_up: bool,

    pub entity_move_up: bool,
    pub entity_move_down: bool,
    pub entity_move_left: bool,
    pub entity_move_right: bool,
}
