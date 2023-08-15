use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::WindowEvent,
        event_loop::{ControlFlow, EventLoopBuilder},
        window::WindowBuilder,
        ContextBuilder,
    },
    index::NoIndices,
    Display, Surface, VertexBuffer,
};
use winit::event::{Event, StartCause};

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    // set up the application window
    let event_loop = EventLoopBuilder::new().build();
    let window_builder = WindowBuilder::new();
    let context_builder = ContextBuilder::new();
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.25],
        },
    ];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

    let indicies = NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140
    
    in vec2 position;

    uniform float t;
    
    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => (),
        };

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame
            .draw(
                &vertex_buffer,
                &indicies,
                &program,
                &uniform! {t: t},
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    });
}
