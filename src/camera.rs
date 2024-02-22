use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::vec2,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::hex::world::view::{PrimaryWorldView, WorldView};

pub struct CameraControllerPlugin;
#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub struct CameraMovementSet;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LockPosition(None));
        app.add_systems(
            Update,
            (zoom_camera, move_camera, restrain_camera)
                .chain()
                .in_set(CameraMovementSet),
        );
    }
}

#[derive(Component)]
pub struct CameraController {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
    pub zoom_target: f32,
    pub cur_zoom: f32,
    /// Keyboard move speed
    pub move_speed: f32,
    /// Mouse move speed
    pub drag_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            min_zoom: 1.0,
            max_zoom: 5.0,
            zoom_speed: 1.0,
            zoom_target: 2.0,
            cur_zoom: 2.0,
            move_speed: 1000.0,
            drag_speed: 1.0,
        }
    }
}

#[derive(Resource)]
struct LockPosition(Option<Vec2>);

fn move_camera(
    mut lock_position: ResMut<LockPosition>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &CameraController)>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut window = windows.single_mut();
    let mut drag = vec2(0., 0.);
    if mouse.pressed(MouseButton::Middle) {
        drag = motion_evr
            .read()
            .fold(drag, |d, ev| d + ev.delta * vec2(-1., 1.));
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        if lock_position.0.is_none() {
            lock_position.0 = window.cursor_position();
        }
    } else {
        if let Some(pos) = lock_position.0 {
            window.set_cursor_position(Some(pos));
            lock_position.0 = None;
        }
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    };
    let mut mov = vec2(0., 0.);
    if key.pressed(KeyCode::ArrowLeft) {
        mov.x -= 1.;
    }
    if key.pressed(KeyCode::ArrowRight) {
        mov.x += 1.;
    }
    if key.pressed(KeyCode::ArrowUp) {
        mov.y += 1.;
    }
    if key.pressed(KeyCode::ArrowDown) {
        mov.y -= 1.;
    }
    for (mut transform, camera) in query.iter_mut() {
        let scale = transform.scale;
        transform.translation +=
            (mov * time.delta_seconds() * camera.move_speed + drag * camera.drag_speed).extend(0.)
                * scale;
    }
}

fn zoom_camera(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    let delta = time.delta_seconds();
    let mut amt = 0.0;
    for ev in scroll_evr.read() {
        amt -= ev.y
            * match ev.unit {
                // also mousepad zoom
                MouseScrollUnit::Line => 0.2,
                // mousepad scrolling should not zoom
                MouseScrollUnit::Pixel => continue,
            };
    }
    for (mut transform, mut camera) in query.iter_mut() {
        camera.zoom_target =
            (camera.zoom_target + amt * camera.zoom_speed).clamp(camera.min_zoom, camera.max_zoom);
        let diff = camera.zoom_target - camera.cur_zoom;
        let speed = diff.abs().sqrt().max(0.2) * delta * camera.zoom_speed * 10.;
        camera.cur_zoom += diff.signum() * speed.min(diff.abs());
        transform.scale = Vec2::splat(2f32.powf(camera.cur_zoom)).extend(1.0);
    }
}

fn restrain_camera(
    world_query: Query<(&Transform, Has<PrimaryWorldView>), With<WorldView>>,
    mut camera_query: Query<&mut Transform, (With<CameraController>, Without<WorldView>)>,
) {
    let priary_pos = world_query
        .iter()
        .filter(|(_, v)| *v)
        .last()
        .unwrap()
        .0
        .translation
        .xy();
    for mut camera_transform in camera_query.iter_mut() {
        let mut min_dst = f32::INFINITY;
        let mut closest_pos = Vec2::ZERO;
        for (world_transform, _) in world_query.iter() {
            let dst = world_transform
                .translation
                .distance(camera_transform.translation);
            if dst < min_dst {
                min_dst = dst;
                closest_pos = world_transform.translation.xy();
            }
        }
        if closest_pos.abs_diff_eq(priary_pos, f32::EPSILON) {
            continue;
        }
        camera_transform.translation =
            (priary_pos + camera_transform.translation.xy() - closest_pos).extend(0.0);
    }
}
