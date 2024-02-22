pub mod view;

use self::view::WorldView;

use super::HexPosition;
use bevy::{math::vec2, prelude::*, utils::HashMap};
use std::mem;
/// Does not add a world resource
pub struct HexWorldPlugin;

impl Plugin for HexWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileChangedEvent>();
        app.add_systems(Update, emit_events);
    }
}

pub struct OutsideMapError;

#[derive(Copy, Clone)]
pub enum Tile {
    Ground,
    Water,
}

#[derive(Resource)]
pub struct HexWorld {
    size: u16,
    tiles: Vec<Tile>,
    changes: Vec<TileChangedEvent>,
}

#[derive(Event)]
pub struct TileChangedEvent {
    pub old_tile: Tile,
    pub new_tile: Tile,
    pub position: HexPosition,
}

impl HexWorld {
    /// Size is map radius, a map of size 1 has 1 tile
    pub fn new(size: u16, base_tile: Tile) -> Self {
        let len = size as usize;
        // took too long to derive
        let len = (len - 1) * len * 3 + 1;
        Self {
            size,
            tiles: vec![base_tile; len],
            changes: Vec::new(),
        }
    }

    pub fn size(&self) -> u16 {
        self.size
    }

    pub fn tile_at(&self, pos: HexPosition) -> Result<Tile, OutsideMapError> {
        Ok(self.tiles[self.tile_index(pos)?])
    }

    pub fn set_tile(&mut self, pos: HexPosition, tile: Tile) -> Result<(), OutsideMapError> {
        let idx = self.tile_index(pos)?;
        self.changes.push(TileChangedEvent {
            old_tile: self.tiles[idx],
            new_tile: tile,
            position: pos,
        });
        self.tiles[idx] = tile;
        Ok(())
    }

    // not sure if this should be placed in this type, feels like a war crime against SRP
    pub fn make_view(&self, commands: &mut Commands, pos: Vec3, tile: Handle<Image>) -> Entity {
        // TODO magic number
        let s = 128.;
        let mut tiles: HashMap<HexPosition, _> = HashMap::new();
        let mut children = Vec::with_capacity(self.tiles.len());
        for r in -(self.size as i32) + 1..self.size as i32 {
            for q in -(self.size as i32) - r.min(0) + 1..(self.size as i32) - r.max(0) as i32 {
                let pos = HexPosition::new(q, r);
                let center = pos.center_2d();
                let tile = commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            anchor: bevy::sprite::Anchor::Custom(vec2(0., 32. / 288. * 0.5)),
                            ..default()
                        },
                        transform: Transform::from_translation((center * s).extend(-center.y)),
                        texture: tile.clone(),
                        ..default()
                    })
                    .id();
                tiles.insert(pos, tile);
                children.push(tile)
            }
        }
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                WorldView { tiles },
            ))
            .push_children(&children)
            .id()
    }

    fn tile_index(&self, pos: HexPosition) -> Result<usize, OutsideMapError> {
        let row = pos.r + self.size as i32 - 1;
        if row > self.size as i32 - 2 || row < 0 {
            return Err(OutsideMapError);
        }
        let col = pos.q + row;
        if col < 0 || col >= row + self.size as i32 {
            return Err(OutsideMapError);
        }
        // sure hope this works, not sure how to debug it...
        Ok((row * self.size as i32 + (row - 1) * row + col) as usize)
    }
}

fn emit_events(mut world: ResMut<HexWorld>, mut writer: EventWriter<TileChangedEvent>) {
    // Use rust, they said. It will be easy, they said... WTF LIFETIMESS
    writer.send_batch(mem::take(&mut world.changes));
    world.changes.clear();
}
