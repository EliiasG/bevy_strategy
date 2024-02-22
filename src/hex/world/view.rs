use bevy::{prelude::*, utils::HashMap};

use crate::hex::HexPosition;

use super::{Tile, TileChangedEvent};
pub struct HexWorldViewPlugin;

impl Plugin for HexWorldViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_world_view);
    }
}

#[derive(Component)]
pub struct WorldView {
    pub tiles: HashMap<HexPosition, Entity>,
}

#[derive(Component)]
pub struct PrimaryWorldView;

fn update_world_view(
    parent_query: Query<&WorldView>,
    mut child_query: Query<&mut Visibility>,
    mut change_reader: EventReader<TileChangedEvent>,
) {
    for worldview in parent_query.iter() {
        for event in change_reader.read() {
            let tile_entity = match worldview.tiles.get(&event.position) {
                None => {
                    error!(
                        "No tile entity at q: {}, r: {}",
                        event.position.q, event.position.r
                    );
                    continue;
                }
                Some(v) => v,
            };
            let mut visibility = match child_query.get_mut(tile_entity.clone()) {
                Err(_) => {
                    error!(
                        "Invalid tile entity at q: {}, r: {}",
                        event.position.q, event.position.r
                    );
                    continue;
                }
                Ok(v) => v,
            };
            // TODO make more sophisticated tile system
            *visibility = match event.new_tile {
                Tile::Ground => Visibility::Inherited,
                Tile::Water => Visibility::Hidden,
            };
        }
    }
}
