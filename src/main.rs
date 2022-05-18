#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::glutin;
    use glium::Surface;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    out vec2 my_color;
    out vec4 pos;

    uniform mat4 matrix;
    void main() {
        my_color = position;
        gl_Position = matrix*vec4(position, 0.0, 1.0);
        pos = gl_Position;
    }
    "#;

    let fragment_shader_src = r#"
    #version 140
    in vec2 my_color;
    out vec4 color;

    void main() {
        color = vec4(my_color, 0.0, 1.0);
    }
    "#;

    let weird_fragment_shader = r#"
    #version 140
    uniform float t;
    in vec4 pos;
    out vec4 color;

    void main() {
        float dist = pos.x*pos.x+pos.y*pos.y+0.1;
        float angle = atan(pos.y/pos.x);
        float loop_x = 0.5*(sin(t*angle)+1.1);
        float loop_y = 0.5*(cos(t*angle)+1.1);
        color = vec4(dist*loop_x, dist*loop_y, 0.0, 1.0);
    }

    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, weird_fragment_shader, None)
            .unwrap();

    let vertex1 = Vertex {
        position: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [1.0, -1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let mut t: f32 = 0.0;
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.2;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            t: t
        };
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
