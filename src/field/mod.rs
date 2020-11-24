use crate::context::Context;

pub struct Field {
    pub field: gpu::Texture2D
}

impl Field {
    pub fn new(context: &Context, size: (usize, usize), dimension: usize) -> Self {
        let format = gpu::TextureFormat::new(gpu::ColorFormat::components(dimension), gpu::Type::F32);
        let field = gpu::Texture2D::allocate(&context.context, size, &format);
        Self { field }
    }

    pub fn from_data(context: &Context, size: (usize, usize), dimension: usize, data: &[f32]) -> Self {
        let format = gpu::TextureFormat::new(gpu::ColorFormat::components(dimension), gpu::Type::F32);
        let field = gpu::Texture2D::from_data(&context.context, size, &format, data, &format);
        Self { field }
    }
}