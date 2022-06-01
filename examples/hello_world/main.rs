mod controller;

use gl;
use glfw;
use glfw::Context;

use controller::Controller;
use cuboid::components::{
    Material,
    Renderer3D,
    Shape
};
use cuboid::core::Shader;
use cuboid::io::CameraController;
use cuboid::utils::types;

const WINDOW_TITLE: &str = "Hello world triangle";

fn main() {
    let triangle_v: Vec<types::V6> = vec![
        [-0.75, -0.75, 0.0, 1.0, 0.0, 0.0],
        [0.75, -0.75, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.75, 0.0, 0.0, 0.0, 1.0],
    ];

    let triangle_i: Vec<u32> = vec![0, 1, 2];

    let (mut window, events, mut glfw_instance) = cuboid::Window::new()
        .width(1000)
        .height(1000)
        .title(WINDOW_TITLE)
        .windowed()
        .build();
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = Shader::new(
        "examples/hello_world/resources/shaders/test.vert",
        "examples/hello_world/resources/shaders/test.frag",
    );
    let material = Material::new(&shader);

    let triangle = Shape::new_with_usage(
        &triangle_v,
        &triangle_i,
        &material,
        &[0, 1],
        gl::STATIC_DRAW,
    );
    renderer.add_item(&triangle);

    let mut wireframe = false;
    let mut controller = Controller::new();

    while !window.should_close() {
        controller.poll_window_events(&mut glfw_instance, &events);
        if controller.esc_pressed {
            window.set_should_close(true);
        }

        if wireframe != controller.wireframe {
            if controller.wireframe {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                println!("LINE")
            } else {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                println!("FILL")
            }
            wireframe = controller.wireframe;
        }

        renderer.clear();
        renderer.render();
        window.swap_buffers();
    }
}
