pub struct Context {
    pub context: gpu::Context,
    pub dimensions: (usize, usize)
}

impl Context {
    pub fn new(dimensions:(usize,usize)) -> Self {
        let display = gpu::ContextDisplay::Window("Fluid".to_string(), dimensions.0, dimensions.1);
        let context = gpu::ContextBuilder::new().with_display(display).build();
        context.make_current().ok();

        Self { context, dimensions }
    }

    pub fn present(&mut self) {
        self.context.swap_buffers().ok();
    }
}