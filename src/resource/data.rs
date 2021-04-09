//! Meshes and Materials
use bevy::prelude::*;

/// Meshes and Materials
pub struct Data {
    pub plane_mesh: Handle<Mesh>,
    pub plane_material: Handle<StandardMaterial>,

    pub cube_mesh: Handle<Mesh>,
    pub cube_material: Handle<StandardMaterial>,
}
