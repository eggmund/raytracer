use image::Rgba;
use na::{Point3, Vector3};

use std::ops::Mul;

#[derive(Debug)]
pub struct Light {
    pub pos: Point3<f32>,
}

impl Light {
    pub fn new(pos: Point3<f32>) -> Light {
        Light {
            pos,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn get_image_rgba(&self) -> Rgba<u8> {
        Rgba([
            (self.r * 255.0).ceil() as u8,
            (self.g * 255.0).ceil() as u8,
            (self.b * 255.0).ceil() as u8,
            (self.a * 255.0).ceil() as u8,
        ])
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color {
            r: (self.r * rhs).min(1.0),
            g: (self.g * rhs).min(1.0),
            b: (self.b * rhs).min(1.0),
            a: self.a,      // Leave alpha alone
        }
    }
}