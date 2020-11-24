use crate::Context;
use crate::fluid::Fluid;

pub struct Presenter {
    pub raster_program: gpu::RasterProgram,
    pub framebuffer: gpu::Framebuffer,
    pub vertex_array_object: gpu::VertexArrayObject
}

impl Presenter {
    pub fn new(context: &Context) -> Self {
        let fragment_shader = gpu::FragmentShader::new(&context.context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
        let vertex_shader = gpu::VertexShader::new(&context.context, include_str!("vertex.glsl")).expect("Couldn't create VertexShader.");
        let raster_program = gpu::RasterProgram::new(&context.context, &fragment_shader, &vertex_shader).expect("Couldn't create RasterProgram.");
        let framebuffer = gpu::Framebuffer::default(&context.context);
        let vertex_array_object = gpu::VertexArrayObject::new(&context.context);
        Self { raster_program, framebuffer, vertex_array_object }
    }

    pub fn present(&mut self, context: &Context, fluid: &Fluid) {
        const VELOCITY_FIELD_LOCATION      : usize = 0;
        const DENSITY_FIELD_LOCATION       : usize = 1;
        const VIEWPORT_DIMENSIONS_LOCATION : usize = 2;
        const FIELD_DIMENSIONS_LOCATION    : usize = 3;
        self.raster_program.bind_image_2d(&fluid.velocity_field, VELOCITY_FIELD_LOCATION);
        self.raster_program.bind_image_2d(&fluid.density_field, DENSITY_FIELD_LOCATION);
        self.raster_program.bind_vec2((context.dimensions.0 as f32, context.dimensions.1 as f32), VIEWPORT_DIMENSIONS_LOCATION);
        self.raster_program.bind_vec2((fluid.dimensions.0 as f32, fluid.dimensions.1 as f32), FIELD_DIMENSIONS_LOCATION);
        self.raster_program.raster(&self.framebuffer, &self.vertex_array_object, gpu::RasterGeometry::Points, 1);
    }
}