extern crate nalgebra as na;

mod ray;
mod objects;
mod scene;
mod lighting;

use na::{Point3, Vector3, Matrix4};
use image::DynamicImage;

use scene::Scene;
use objects::{Sphere, HorizontalPlane};
use lighting::{Color, Light};

const DIMS: (u32, u32) = (800, 600);

struct MainState {
    main_scene: Scene,
}

impl MainState {
    fn new() -> MainState {
        let mut main_scene = Scene::new(DIMS.0, DIMS.1, 90.0,
            vec![
                Box::new(Sphere {
                    centre: Point3::new(0.0, 1.0, -0.1),
                    radius: 1.0,
                    color: Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                    },
                }),
                Box::new(Sphere {
                    centre: Point3::new(-2.0, -2.0, -7.0),
                    radius: 1.0,
                    color: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                }),
                Box::new(Sphere {
                    centre: Point3::new(2.0, -2.0, 4.0),
                    radius: 1.0,
                    color: Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                    },
                }),

                Box::new(HorizontalPlane {
                    y: -6.0,
                    color: Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                    },
                }),
            ],
            vec![
                Light::new(Point3::new(-4.0, 5.0, -3.0), 1.0), // 
                Light::new(Point3::new(4.0, 5.0, -3.0), 1.0),
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