pub mod world;

use bevy::{math::vec2, prelude::*};

const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

/// Position is pointy hex
#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub struct HexPosition {
    pub q: i32,
    pub r: i32,
}

impl HexPosition {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    // source: https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
    /// from regular 2d cord to hex cord. Assumes value is in units of outer radius
    pub fn from_2d(point: Vec2) -> Self {
        let q = SQRT_3 / 3. * point.x - 1. / 3. * point.y;
        let r = 2. / 3. * point.y;
        axial_round(q, r)
    }

    /// returns the centre as a regular 2d cord, result is in units of outer radius
    pub fn center_2d(&self) -> Vec2 {
        let x = SQRT_3 * self.q as f32 + SQRT_3 * 0.5 * self.r as f32;
        let y = 1.5 * self.r as f32;
        vec2(x, y)
    }
}

// source: https://www.redblobgames.com/grids/hexagons/#rounding
fn axial_round(q: f32, r: f32) -> HexPosition {
    let s = -q - r;
    let qr = q.round();
    let rr = r.round();
    let sr = s.round();
    let qd = (qr - q).abs();
    let rd = (rr - r).abs();
    let sd = (sr - s).abs();
    if qd > rd && qd > sd {
        HexPosition::new((-rr - sr) as i32, r as i32)
    } else if rd > sd {
        HexPosition::new(qr as i32, (-qr - sr) as i32)
    } else {
        HexPosition::new(qr as i32, rr as i32)
    }
}
