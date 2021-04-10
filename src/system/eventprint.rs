//! Print picking events to stdout.
use bevy::prelude::*;
use bevy_mod_picking;

/// Print picking events to stdout.
pub fn event_print(mut picking_events: EventReader<bevy_mod_picking::PickingEvent>) {
    for event in picking_events.iter() {
        println!("Event: {:?}", event);
    }
}
