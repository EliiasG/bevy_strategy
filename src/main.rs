pub mod camera;
pub mod hex;

use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::*;
use hex::{
    world::{
        view::{HexWorldViewPlugin, PrimaryWorldView},
        HexWorld, HexWorldPlugin, Tile,
    },
    HexPosition,
};

const SIZE: i32 = 15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_linear()))
        .add_plugins((CameraControllerPlugin, HexWorldPlugin, HexWorldViewPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb_u8(77, 155, 230)))
        .insert_resource(Msaa::Off)
        .insert_resource(HexWorld::new(SIZE as u16, Tile::Water))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<HexWorld>) {
    let tile = asset_server.load("tile.png");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(vec3(10., 10., 0.)),
            ..default()
        },
        CameraController::default(),
    ));
    let mut q = 2 * SIZE - 1;
    let mut r = -SIZE + 1;
    let mut s = -q - r;
    let primary = world.make_view(&mut commands, vec3(0., 0., 0.), tile.clone());
    commands.entity(primary).insert(PrimaryWorldView);
    for _ in 0..6 {
        (q, r, s) = (-r, -s, -q);
        let pos = HexPosition::new(q, r).center_2d();
        println!("{q} {r} {s} pos {}, {}", pos.x, pos.y);
        world.make_view(
            &mut commands,
            pos.xyy() * vec3(1., 1., -0.01) * 128.,
            tile.clone(),
        );
    }
}
