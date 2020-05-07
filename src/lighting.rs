use image::Rgba;
use na::{Point3, Vector3};

use std::ops::{Mul, MulAssign};

#[derive(Debug)]
pub struct Light {
    pub pos: Point3<f32>,
    pub luminance: f32,     // From 1 to 0
}

impl Light {
    pub fn new(pos: Point3<f32>, luminance: f32) -> Light {
        Light {
            pos,
            luminance,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn grey_from_float(a: f32) -> Color {
        Color::new(a, a, a)
    }

    pub fn get_image_rgba(&self) -> Rgba<u8> {
        Rgba([
            (self.r * 255.0).ceil() as u8,
            (self.g * 255.0).ceil() as u8,
            (self.b * 255.0).ceil() as u8,
            255,
        ])
    }

    // Fog like affect
    #[inline]
    pub fn fade_due_to_render_distance(ray_col: &Color, distance_travelled: f32) -> Color {
        use crate::ray::RAY_MAX_TRAVEL_DISTANCE;

        let fade_amount = distance_travelled/RAY_MAX_TRAVEL_DISTANCE;
        *ray_col
    }

    pub fn is_black(&self) -> bool {
        (self.r == 0.0) && (self.g == 0.0) && (self.b == 0.0)
    }
}


impl Mul<&Color> for Color {
    type Output = Self;

    fn mul(self, rhs: &Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

// For blending
impl MulAssign<&Color> for Color {
    fn mul_assign(&mut self, rhs: &Color) {
        *self = (*self) * rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color {
            r: (self.r * rhs).min(1.0),
            g: (self.g * rhs).min(1.0),
            b: (self.b * rhs).min(1.0),
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = (self.r * rhs).min(1.0);
        self.g = (self.g * rhs).min(1.0);
        self.b = (self.b * rhs).min(1.0);
    }
}