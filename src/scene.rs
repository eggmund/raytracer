use image::{DynamicImage, GenericImage, Rgba, Rgb, Pixel};
use na::{Point3, Perspective3, Vector3};

use std::f32::consts::PI;

use crate::objects::{Sphere, Object};
use crate::ray::{Ray, RAY_MAX_TRAVEL_DISTANCE, RAY_REFLECT_LIMIT};
use crate::lighting::{Light, Color};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,       // In DEGREES
    pub camera_pos: Point3<f32>,
    pub perspective: Perspective3<f32>,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(width: u32, height: u32, camera_pos: Point3<f32>, fov: f32, objects: Vec<Box<dyn Object>>, lights: Vec<Light>) -> Scene {
        Scene {
            width,
            height,
            camera_pos,
            perspective: Perspective3::new(width as f32/height as f32, PI/2.0, 1.0, RAY_MAX_TRAVEL_DISTANCE),
            fov,
            objects,
            lights,
        }
    }

    pub fn render(&self) -> DynamicImage {
        // Check if a point is in shadow
        #[inline]
        fn check_is_in_shadow(boundary_position: &Point3<f32>, objects: &Vec<Box<dyn Object>>, object_index: usize, light_dot: f32, light_normal: &Vector3<f32>) -> bool {
            // If dot product < 0, then it is a surface away from the light so will be in shadow, so there won't be any objects blocking
            // the light to this point, since it is not in light
            if light_dot < 0.0 {
                true
            } else { // Cast ray towards light, and if it hits something then it is in shadow
                let mut ray = Ray::new(*boundary_position, *light_normal);
                let (hit_data, distance_traveled) = ray.march_until_hit(objects, &[object_index]);
                if hit_data.is_some() { // If it hits something -> in shadow
                    true
                } else {
                    false
                }
            }
        }

        
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0, 0, 0, 0);

        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray = Ray::create_prime(x, y, self);

                let mut color = Color::white();

                for i in 0..RAY_REFLECT_LIMIT+1 {
                    let (hit_info, distance_traveled) = ray.march_until_hit(&self.objects, &[]);
                    if let Some((object_index, boundary_position)) = hit_info { // If it hit something, add color
                        let light_normal = (self.lights[0].pos - boundary_position).normalize(); // vector from where the ray hit the object to light
                        let surface_normal = self.objects[object_index].get_normal(boundary_position);
            
                        let light_dot = surface_normal.dot(&light_normal);      

                        if i == 0 && check_is_in_shadow(&boundary_position, &self.objects, object_index, light_dot, &light_normal) { // Check if it is in shadow before continuing
                            color = Color::black();
                            break;
                        }

                        // Blend color
                        color *= self.objects[object_index].get_color_ref();
                        color *= light_dot;
                        if self.objects[object_index].get_reflectance() == 0.0 { // stop reflecting
                            break;
                        } else {
                            color *= self.objects[object_index].get_reflectance();
                            ray.reflect(self.objects[object_index].get_normal(boundary_position));
                        }
                    } else {    // Nothing to reflect off of
                        break;
                    }
                }

                image.put_pixel(x, y, color.get_image_rgba());
            }
        }

        image
    }
}