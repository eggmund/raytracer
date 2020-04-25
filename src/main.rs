extern crate nalgebra as na;

mod ray;
mod objects;
mod scene;
mod lighting;

use na::{Point3, Vector3};
use image::DynamicImage;

use scene::Scene;
use objects::{Sphere, HorizontalPlane};
use lighting::{Color, Light};

const DIMS: (u32, u32) = (1000, 800);

struct MainState {
    main_scene: Scene,
}

impl MainState {
    fn new() -> MainState {
        let main_scene = Scene::new(DIMS.0, DIMS.1, Point3::origin(), 90.0,
            vec![
                Box::new(Sphere {
                    centre: Point3::new(0.0, 0.0, -5.0),        // 5 units away from camera
                    radius: 1.0,
                    color: Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },
                }),
                Box::new(Sphere {
                    centre: Point3::new(5.0, -2.0, -10.0),        // 5 units away from camera
                    radius: 1.0,
                    color: Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                }),
                Box::new(HorizontalPlane {
                    y: -6.0,
                    color: Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                }),
            ],
            vec![
                Light::new(Point3::new(0.0, 10.0, -3.0))
            ],
        );

        MainState {
            main_scene,
        }
    }

    fn render(&self) -> DynamicImage {
        self.main_scene.render()
    }
}


fn main() {
    use std::time::Instant;

    let state = MainState::new();

    let render_start_time = Instant::now();
    let image = state.render();
    println!("Time taken: {:?}", Instant::now().duration_since(render_start_time));

    image.save("out.png").unwrap();
}