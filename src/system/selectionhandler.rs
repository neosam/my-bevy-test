//! Handle selection of entities.
use crate::component;
use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use bevy_mod_picking::SelectionEvent;

/// Handle selection of entities.
pub fn selection_handler(mut pick_events: EventReader<PickingEvent>, mut commands: Commands) {
    for pick_event in pick_events.iter() {
        match &pick_event {
            &PickingEvent::Selection(SelectionEvent::JustSelected(entity)) => {
                let mut entity = commands.entity(*entity);
                entity.insert(component::Selected);
            }
            &PickingEvent::Selection(SelectionEvent::JustDeselected(entity)) => {
                let mut entity = commands.entity(*entity);
                entity.remove::<component::Selected>();
            }
            _ => (),
        }
    }
}
