use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    mins: Vec2,
    maxs: Vec2,
}

impl Aabb {
    pub fn new(mins: Vec2, maxs: Vec2) -> Self {
        Self { mins, maxs }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            mins: self.mins.min(other.mins),
            maxs: self.maxs.max(other.maxs),
        }
    }

    pub fn extend(&self, point: Vec2) -> Self {
        Self {
            mins: self.mins.min(point),
            maxs: self.maxs.max(point),
        }
    }

    pub fn area(&self) -> f32 {
        let diag = self.maxs - self.mins;
        diag.x * diag.y
    }

    // fn from_half_extents(center: Vec2, half_extents: Vec2) -> Self {
    //     Self {
    //         mins: center - half_extents,
    //         maxs: center + half_extents,
    //     }
    // }

    // fn center(&self) -> Vec2 {
    //     (self.mins + self.maxs) / 2.0
    // }

    // fn half_extents(&self) -> Vec2 {
    //     (self.maxs - self.mins) / 2.0
    // }

    // fn intersects(&self, other: &Self) -> bool {
    //     self.mins.x < other.maxs.x
    //         && self.maxs.x > other.mins.x
    //         && self.mins.y < other.maxs.y
    //         && self.maxs.y > other.mins.y
    // }

    // fn area(&self) -> f64 {
    //     let half_extents = self.half_extents();
    //     half_extents.x * half_extents.y
    // }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            mins: Vec2::new(f32::MAX, f32::MAX),
            maxs: Vec2::new(f32::MIN, f32::MIN),
        }
    }
}

impl Add<Vec2> for Aabb {
    type Output = Aabb;

    fn add(self, other: Vec2) -> Self::Output {
        Self::Output {
            mins: self.mins + other,
            maxs: self.maxs + other,
        }
    }
}

impl Sub<Vec2> for Aabb {
    type Output = Aabb;

    fn sub(self, other: Vec2) -> Self::Output {
        Self::Output {
            mins: self.mins - other,
            maxs: self.maxs - other,
        }
    }
}

impl AddAssign<Vec2> for Aabb {
    fn add_assign(&mut self, rhs: Vec2) {
        self.mins += rhs;
        self.maxs += rhs;
    }
}

impl SubAssign<Vec2> for Aabb {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.mins -= rhs;
        self.maxs -= rhs;
    }
}
