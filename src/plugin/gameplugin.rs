//! One plugin which sets everything up for the whole game.
use crate::event;
use crate::resource;
use crate::state;
use crate::system;
use bevy::prelude::*;
use bevy_mod_picking;

/// One plugin which sets everything up for the whole game.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Resources and events
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(resource::CameraPosition(0.0, 0.0, 10.0))
            .insert_resource(resource::InputCommands::default())
            .add_event::<event::InputEvent>();

        // Plugins
        app.add_plugins(DefaultPlugins)
            .add_plugin(bevy_mod_picking::PickingPlugin)
            .add_plugin(bevy_mod_picking::InteractablePickingPlugin);
        //.add_plugin(bevy_mod_picking::HighlightablePickingPlugin);

        // Web renderer
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // Diagnostics (print FPS)
        //app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        //    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

        // Ingame systems
        app.add_system_set(
            SystemSet::on_enter(state::State::InGame).with_system(system::ingame_setup.system()),
        );
        app.add_system_set(
            SystemSet::on_update(state::State::InGame)
                .with_system(system::user_input.system())
                .with_system(system::camera_movement.system())
                .with_system(system::camera_positioning.system())
                .with_system(system::cube_spawner.system())
                .with_system(system::selection_handler.system())
                .with_system(system::movement.system())
                .with_system(system::event_print.system()),
        );

        app.add_system_set(
            SystemSet::on_enter(state::State::Menu).with_system(system::menu_setup.system()),
        );

        // Set initial state
        app.add_state(state::State::Menu);
    }
}
