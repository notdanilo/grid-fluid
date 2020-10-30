use crate::Context;

pub struct Fluid {
    pub velocity: gpu::Texture2D,
    pub density: gpu::Texture2D,
    pub viscosity: f32,
    pub diffusion: f32,
    pub delta_time: f32
}

impl Fluid {
    pub fn new(context: &Context, dimensions: (usize, usize), delta_time: f32, diffusion: f32, viscosity: f32) -> Self {
        let color_format = gpu::ColorFormat::RGBA;
        let component_type = gpu::Type::F32;
        let velocity_format  = gpu::TextureFormat::new(color_format, component_type);
        let velocity = gpu::Texture2D::allocate(&context.context, dimensions, &velocity_format);

        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let density_format  = gpu::TextureFormat::new(color_format, component_type);
        let density = gpu::Texture2D::allocate(&context.context, dimensions, &density_format);

        Self { delta_time, velocity, density, diffusion, viscosity }
    }
}