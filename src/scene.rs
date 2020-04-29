use image::{DynamicImage, GenericImage, Rgba, Pixel};
use na::{Point3, Perspective3, Vector3, Matrix4, Isometry3};

use std::f32::consts::PI;

use crate::objects::{Sphere, Object};
use crate::ray::{Ray, HitData, RAY_MAX_TRAVEL_DISTANCE, RAY_REFLECT_LIMIT, RAY_HIT_THRESHOLD};
use crate::lighting::{Light, Color};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,       // In DEGREES
    pub perspective: Perspective3<f32>,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f32, objects: Vec<Box<dyn Object>>, lights: Vec<Light>) -> Scene {
        Scene {
            width,
            height,
            perspective: Perspective3::new(width as f32/height as f32, PI/2.0, 1.0, RAY_MAX_TRAVEL_DISTANCE),
            fov,
            objects,
            lights,
        }
    }

    // pub fn move_camera(&mut self, transform: Isometry3<f32>) {
    //     // self.camera_pos += d_pos;
    //     self.perspective = Perspective3::from_matrix_unchecked(self.perspective.as_matrix() * transform);
    // }

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray = Ray::create_prime(x, y, self);

                for i in 0..RAY_REFLECT_LIMIT+1 {
                    let (hit_data, distance_traveled) = ray.march_until_hit(&self.objects, &[]);
                    if let Some(hit_data) = hit_data { // If it hit something, add color
                        let object_hit = &self.objects[hit_data.object_index];
                        let reflectance = object_hit.get_reflectance();

                        let mut hit_color = *(object_hit.get_color_ref());
                        let shade = self.shade(&hit_data);

                        hit_color *= shade;
                        ray.color *= &hit_color;

                        if self.objects[hit_data.object_index].get_reflectance() == 0.0 {
                            break;
                        }
                        ray.reflect(self.objects[hit_data.object_index].get_normal(&hit_data.point_of_contact));
                    } else {    // Reached distance limit - nothing to reflect off of
                        break;
                    }
                }

                image.put_pixel(x, y, ray.color.get_image_rgba());
            }
        }

        image
    }

    // Check if a point is in shadow. Retuns 0 -> 1 of how much light
    #[inline]
    fn shade(&self, hit_data: &HitData) -> f32 {
        let mut light_amount = 0.0;     // Assume complete darkness unless there are lights

        // Get the surface normal at the point of hit
        let surface_normal = self.objects[hit_data.object_index].get_normal(&hit_data.point_of_contact);
        
        let light_dot1 = (self.lights[0].pos - hit_data.point_of_contact).dot(&surface_normal);

        for light in self.lights.iter() {
            // Get a normal vector going from the surface to the light
            let light_norm = (light.pos - hit_data.point_of_contact).normalize();
            // Amount of light illuminating surface
            let light_dot = light_norm.dot(&surface_normal);
            
            if light_dot > 0.0 {   // If it is less than 0, then light is away from surface, so not illuminating
                // new ray emitted from boundary position towards light, if it hits something then it is in shadow.
                let mut shadow_ray = Ray::new(hit_data.point_of_contact, light_norm);
                // March, ignoring the parent object
                let (hit_data, distance_traveled) = shadow_ray.march_until_hit(&self.objects, &[hit_data.object_index]);

                // If it didn't hit anything, and didn't hit plane, it is light
                match hit_data {
                    Some(hit_data) => {
                        if hit_data.object_type_name == "HorizontalPlane" {
                            light_amount += light_dot * light.luminance;
                        }
                    },
                    None => light_amount += light_dot * light.luminance,
                }
            }
        }
        light_amount/self.lights.len() as f32
    }   
}