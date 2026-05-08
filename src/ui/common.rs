use bevy::prelude::*;

#[derive(Message, Default)]
pub struct MapResize;

#[derive(Message, Default)]
pub struct MapSyncEndpoints;
