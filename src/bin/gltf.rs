use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use support::{
    app::{run_application, App},
    shader::ShaderProgram,
    world::{load_gltf, World},
};

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    vao: u32,
    world: World,
}

impl DemoApp {
    fn load_shaders(&mut self) -> Result<()> {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/gltf/gltf.vs.glsl")?
            .fragment_shader_file("assets/shaders/gltf/gltf.fs.glsl")?
            .link();
        Ok(())
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.world.initialize()?;
        load_gltf("assets/models/helmet.glb", &mut self.world)?;
        self.load_shaders()?;
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
        Ok(())
    }

    fn render(&mut self, _time: f32) -> Result<()> {
        let background_color: [GLfloat; 4] = [0.4, 0.6, 0.9, 1.0];
        self.shader_program.use_program();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Gltf Renderer")
}
