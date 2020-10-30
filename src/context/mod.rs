pub struct Context {
    pub context: gpu::Context,
    pub vertex_shader: gpu::VertexShader,
    pub vertex_array_object: gpu::VertexArrayObject
}

impl Context {
    pub fn new(dimensions:(usize,usize)) -> Self {
        let display = gpu::ContextDisplay::Window("Fluid".to_string(), dimensions.0, dimensions.1);
        let context = gpu::ContextBuilder::new().with_display(display).build();
        let vertex_shader = gpu::VertexShader::new(&context, include_str!("vertex.glsl")).expect("Couldn't create VertexShader.");
        let vertex_array_object = gpu::VertexArrayObject::new(&context);

        context.make_current().ok();
        Self { context, vertex_shader, vertex_array_object }
    }
}