use na::{Point3, Point2, Vector3, Vector2, Perspective3};

use crate::scene::Scene;
use crate::objects::Object;

pub const RAY_MAX_TRAVEL_DISTANCE: f32 = 100.0; // Distance before ray stops marching
pub const RAY_HIT_THRESHOLD: f32 = 0.001; // Minimum distance from object before it is considered hit.
pub const RAY_REFLECT_LIMIT: usize = 1;      // Number of times ray can be reflected

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
        let screen_point = Point2::new(x as f32, y as f32);

        // From https://www.nalgebra.org/cg_recipes/#screen-space-to-view-space
        // Normalize and make far and near points (in front and behind camera). ndc -> normalised device constant
        let ndc_point = Point2::new((screen_point.x as f32 / scene.width as f32) * 2.0 - 1.0, 1.0 - (screen_point.y / scene.height as f32) * 2.0);
        let near_ndc_point = Point3::new(ndc_point.x, ndc_point.y, -1.0);
        let far_ndc_point  = Point3::new(ndc_point.x, ndc_point.y, 1.0);

        // Unproject them to view-space.
        let near_view_point = scene.perspective.unproject_point(&near_ndc_point);
        let far_view_point  = scene.perspective.unproject_point(&far_ndc_point);

        // Compute the view-space line parameters.
        let line_direction = (far_view_point - near_view_point).normalize();
        let emission_pos = Point3::new(near_view_point.x, near_view_point.y, near_view_point.z);

        Ray::new(
            near_view_point,
            line_direction,
        )
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

    // Change direction of ray based on a surface normal given
    pub fn reflect(&mut self, surface_normal: Vector3<f32>) {
        // d_n = d - 2n(d . n)
        self.direction -= 2.0 * surface_normal * self.direction.dot(&surface_normal);
    }
}