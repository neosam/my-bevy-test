use bevy::prelude::*;

mod component;
mod event;
mod plugin;
mod resource;
mod system;

fn main() {
    let mut builder = App::build();
    builder.add_plugin(plugin::GamePlugin).run();
}
