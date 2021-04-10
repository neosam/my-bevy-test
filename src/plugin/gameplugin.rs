//! One plugin which sets everything up for the whole game.
use crate::event;
use crate::resource;
use crate::system;
use bevy::prelude::*;
use bevy_mod_picking;

/// One plugin which sets everything up for the whole game.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(resource::CameraPosition(0.0, 0.0, 10.0))
            .insert_resource(resource::InputCommands::default())
            .add_event::<event::InputEvent>()
            .add_plugins(DefaultPlugins)
            .add_plugin(bevy_mod_picking::PickingPlugin)
            .add_plugin(bevy_mod_picking::InteractablePickingPlugin);
        //.add_plugin(bevy_mod_picking::HighlightablePickingPlugin);
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
        //app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        //    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(system::setup.system())
            .add_system(system::user_input.system())
            .add_system(system::camera_movement.system())
            .add_system(system::camera_positioning.system())
            .add_system(system::cube_spawner.system())
            .add_system(system::selection_handler.system())
            .add_system(system::movement.system())
            .add_system(system::event_print.system());
    }
}
