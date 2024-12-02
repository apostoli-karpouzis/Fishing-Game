use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentInterface {
    #[default]
    Overworld,
    Fishing,
    Shop,
    Inventory
}