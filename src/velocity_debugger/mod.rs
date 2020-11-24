use crate::context::Context;

pub struct VelocityDebugger {
    raster: gpu::RasterProgram,
    vertex_array_object: gpu::VertexArrayObject,
    buffer: gpu::Buffer,
    framebuffer: gpu::Framebuffer
}

impl VelocityDebugger {
    pub fn new(context: &Context) -> Self {
        let data: Vec<f32> = vec![
             -1.0,  0.25,
              1.0,  0.00,
             -1.0, -0.25
        ];
        let buffer = gpu::Buffer::from_data(&context.context, &data);
        let mut vertex_array_object = gpu::VertexArrayObject::new(&context.context);
        vertex_array_object.set_vertex_buffer(&buffer, 0, 2);
        vertex_array_object.set_vertices(3);
        let vertex = gpu::VertexShader::new(&context.context, include_str!("vertex.glsl")).expect("Couldn't create fragment.");
        let fragment = gpu::FragmentShader::new(&context.context, include_str!("fragment.glsl")).expect("Couldn't create fragment.");
        let raster = gpu::RasterProgram::new(&context.context, &fragment, &vertex).expect("Couldn't create program.");
        let framebuffer = gpu::Framebuffer::default(&context.context);
        Self { buffer, vertex_array_object, raster, framebuffer }
    }

    pub fn debug(&self, velocity_field: &gpu::Texture2D) {
        self.raster.bind_vec2((0.0, 0.0), 0);
        self.raster.program.bind_image_2d(velocity_field, 1);
        self.raster.raster(&self.framebuffer, &self.vertex_array_object, gpu::RasterGeometry::Triangles, self.vertex_array_object.get_vertices());
    }
}