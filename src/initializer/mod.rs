use crate::Context;
use crate::fluid::Fluid;

pub struct Initializer {
    pub raster_program: gpu::RasterProgram
}

impl Initializer {
    pub fn new(context: &Context) -> Self {
        let fragment_shader = gpu::FragmentShader::new(&context.context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
        let raster_program = gpu::RasterProgram::new(&context.context, &fragment_shader, &context.vertex_shader).expect("Couldn't create RasterProgram.");
        Self { raster_program }
    }

    pub fn initialize(&mut self, context: &Context, fluid: &Fluid) {
        self.raster_program.raster(&fluid.framebuffer, &context.vertex_array_object, gpu::RasterGeometry::Points, 1);
        context.context.swap_buffers().ok();

    }
}