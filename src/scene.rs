use image::{DynamicImage, GenericImage, Rgba, Rgb, Pixel};
use na::{Point3};

use crate::objects::{Sphere, Object};
use crate::ray::Ray;
use crate::lighting::{Light, Color};

const MAX_SHADOW_CALC_DIST: f32 = 100.0; // max distance away from camera to render shadows for

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,       // In DEGREES
    pub camera_pos: Point3<f32>,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(width: u32, height: u32, camera_pos: Point3<f32>, fov: f32, objects: Vec<Box<dyn Object>>, lights: Vec<Light>) -> Scene {
        Scene {
            width,
            height,
            camera_pos,
            fov,
            objects,
            lights,
        }
    }

    pub fn render(&self) -> DynamicImage {
        // Check if a point is in shadow
        fn render_shadow(last_ray_end_position: Point3<f32>, light_pos: Point3<f32>, objects: &Vec<Box<dyn Object>>, parent_object: usize) -> Color {
            // println!("Casting shadow: {:?}", last_ray_end_position);
            let light_normal = (light_pos - last_ray_end_position).normalize(); // vector from where the ray hit the object to light
            let surface_normal = objects[parent_object].get_normal(last_ray_end_position);

            let light_dot = surface_normal.dot(&light_normal);

            // If dot product < 0, then it is a surface away from the light, so there won't be any objects blocking
            // the light to this point, since it is not in light
            if light_dot < 0.0 {
                Color::black()
            } else { // Cast ray towards light, and if it hits something then it is in shadow
                let mut ray = Ray::new(last_ray_end_position, light_normal);
                let (hit_data, distance_traveled) = ray.march_until_hit(objects, &[parent_object]);
                if hit_data.is_some() { // If it hits something -> in shadow
                    Color::black()
                } else {
                    *(objects[parent_object].get_color_ref()) * light_dot.abs()
                }
            }
            

            // *(objects[parent_object].get_color_ref()) * light_amount
        }
        
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0, 0, 0, 0);

        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray = Ray::create_prime(x, y, self);

                let (hit_info, distance_traveled) = ray.march_until_hit(&self.objects, &[]);

                if let Some((object_index, boundary_position)) = hit_info { // If it hit something, render
                    let render_color = render_shadow(boundary_position, self.lights[0].pos, &self.objects, object_index);
                    image.put_pixel(x, y, render_color.get_image_rgba())
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }

        image
    }
}
