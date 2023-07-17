use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize, Resource, Default)]
pub struct StarFieldConfig {
    strings: Strings,
}

#[derive(Debug,Deserialize, Serialize, Default)]
pub struct Strings {
    title: String,
}

