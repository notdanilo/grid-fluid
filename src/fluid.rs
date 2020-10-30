use crate::Context;

pub struct Fluid {
    pub velocity: gpu::Texture2D,
    pub previous_velocity: gpu::Texture2D,
    pub density: gpu::Texture2D,
    pub previous_density: gpu::Texture2D,
    pub viscosity: f32,
    pub diffusion: f32
}

impl Fluid {
    pub fn new(context: &Context, dimensions: (usize, usize), diffusion: f32, viscosity: f32) -> Self {
        let color_format = gpu::ColorFormat::RGBA;
        let component_type = gpu::Type::F32;
        let velocity_format  = gpu::TextureFormat::new(color_format, component_type);
        let velocity = gpu::Texture2D::allocate(&context.context, dimensions, &velocity_format);
        let previous_velocity = gpu::Texture2D::allocate(&context.context, dimensions, &velocity_format);

        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let density_format  = gpu::TextureFormat::new(color_format, component_type);
        let density = gpu::Texture2D::allocate(&context.context, dimensions, &density_format);
        let previous_density = gpu::Texture2D::allocate(&context.context, dimensions, &density_format);

        Self { velocity, previous_velocity, density, previous_density, diffusion, viscosity }
    }
}