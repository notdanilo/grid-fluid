use crate::Context;
use crate::fluid::Fluid;

pub struct Presenter {
    pub raster_program: gpu::RasterProgram,
    pub framebuffer: gpu::Framebuffer
}

impl Presenter {
    pub fn new(context: &Context) -> Self {
        let fragment_shader = gpu::FragmentShader::new(&context.context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
        let raster_program = gpu::RasterProgram::new(&context.context, &fragment_shader, &context.vertex_shader).expect("Couldn't create RasterProgram.");
        let framebuffer = gpu::Framebuffer::default(&context.context);
        Self { raster_program, framebuffer }
    }

    pub fn present(&mut self, context: &Context, fluid: &Fluid) {
        self.raster_program.bind_image_2d(&fluid.velocity, 0);
        self.raster_program.bind_image_2d(&fluid.density, 1);
        self.raster_program.raster(&self.framebuffer, &context.vertex_array_object, gpu::RasterGeometry::Points, 1);
        context.context.swap_buffers().ok();
    }
}