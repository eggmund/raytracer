use na::{Point3, Vector3};

use crate::scene::Scene;
use crate::objects::Object;

const RAY_MAX_TRAVEL_DISTANCE: f32 = 100.0; // Distance before ray stops marching
const RAY_MAX_SHADOW_DISTANCE: f32 = 1.0;
pub const RAY_HIT_THRESHOLD: f32 = 0.01; // Minimum distance from object before it is considered hit.

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    position: Point3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin.clone(),
            direction,
            position: origin,
        }
    }

    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // Camera is aligned along -z axis (convention). +x = right, +y = up
        // 90 -> 45 degrees multiplier for each vector depending on position (hence -1.0 -> 1.0 camera coords)
        let fov_adjustment = (scene.fov.to_radians()/2.0).tan();
        let aspect_ratio = scene.width as f32/scene.height as f32;

        // Divide camera "sensor" into pixels, with normal vectors coming off of the centre of each
        // x + 0.5 = pixel centre. pixel centre/screen width = normalised position. * 2 - 1 = adjustment
        // Then multiply by fov adjustment  and aspect ratio adjustment
        let sensor_x = (((x as f32 + 0.5)/scene.width as f32) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - ((y as f32 + 0.5)/scene.height as f32) * 2.0) * fov_adjustment;

        Ray::new(scene.camera_pos, Vector3::new(sensor_x, sensor_y, -1.0).normalize())
    }
    
    fn get_closest_object_estimate(&self, objects: &Vec<Box<dyn Object>>, ignore: &[usize]) -> (Option<f32>, Option<usize>) {
        let mut closest = (None, None);

        for (i, object) in objects.iter().enumerate().filter(|(i, _)| !ignore.contains(i)) {
            let distance_estimate = object.distance_estimate(self.position);
            if closest.0 == None || closest.0 > Some(distance_estimate) {
                closest = (Some(distance_estimate), Some(i));
            }
        }
        closest
    }

    // Returns index of object hit first if it did hit, and the position of hit
    // ignore is used when calculating shadows, telling it to ignore the parent object
    pub fn march_until_hit(&mut self, objects: &Vec<Box<dyn Object>>, ignore: &[usize]) -> (Option<(usize, Point3<f32>)>, f32) {    // returns (Some(object index, point of hit), distance traveled)
        // Move forwards at least once, so that if radiating from the surface of an object it doesn't just sit there
        self.position = self.origin + self.direction * RAY_HIT_THRESHOLD;
        let mut distance_traveled = RAY_HIT_THRESHOLD;

        loop {
            if let (Some(smallest_distance_estimate), Some(index)) = self.get_closest_object_estimate(objects, ignore) {
                // if smallest_distance_estimate > 100.0 {
                //     println!("Closest approach: {:?}", closest.0)
                // }

                if smallest_distance_estimate < RAY_HIT_THRESHOLD {
                    return (Some((index, self.position)), distance_traveled)
                } else if distance_traveled > RAY_MAX_TRAVEL_DISTANCE {
                    return (None, distance_traveled)     // Didn't hit any
                }
    
                self.position += self.direction * smallest_distance_estimate;
                distance_traveled += smallest_distance_estimate;
            } else {
                // No objects to hit
                return (None, distance_traveled)
            }
        }
    }
}