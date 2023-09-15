use std::time::{Duration, Instant};

use glium::{
    glutin::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoopBuilder,
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::PrimitiveType,
    program,
    texture::RawImage2d,
    uniform,
    uniforms::{MagnifySamplerFilter, Sampler},
    Blend, Display, DrawParameters, IndexBuffer, Surface, Texture2d, VertexBuffer,
};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let wb = WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_inner_size(PhysicalSize::new(500, 500))
        .with_title("Mamfred");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    let image = image::open("./image.png").unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let opengl_texture = Texture2d::new(&display, image).unwrap();
    let vertex_buffer = VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
        ],
    )
    .unwrap();
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]).unwrap();
    let program = program!(&display,
        140 => {
            vertex: include_str!("shaders/140.vert"),
            fragment: include_str!("shaders/140.frag")
        },
        110 => {
            vertex: include_str!("shaders/110.vert"),
            fragment: include_str!("shaders/110.frag"),
        },
        100 => {
            vertex: include_str!("shaders/100.vert"),
            fragment: include_str!("shaders/100.frag"),
        },
    )
    .unwrap();
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    return;
                }
                _ => (),
            },
            _ => (),
        }
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: Sampler::new(&opengl_texture).magnify_filter(MagnifySamplerFilter::Nearest)
        };
        let next_frame_time = Instant::now() + Duration::from_millis(20);
        control_flow.set_wait_until(next_frame_time);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
        target.finish().unwrap();
    });
}
