use crate::Context;

pub struct Fluid {
    pub velocity_field: gpu::Texture2D,
    pub previous_velocity_field: gpu::Texture2D,
    pub density_field: gpu::Texture2D,
    pub previous_density_field: gpu::Texture2D,
    pub viscosity: f32,
    pub diffusion: f32,
    pub dimensions: (usize, usize)
}

impl Fluid {
    pub fn new(context: &Context, dimensions: (usize, usize), diffusion: f32, viscosity: f32) -> Self {
        let color_format = gpu::ColorFormat::RGBA;
        let component_type = gpu::Type::F32;
        let velocity_field_format = gpu::TextureFormat::new(color_format, component_type);
        let velocity_field = gpu::Texture2D::allocate(&context.context, dimensions, &velocity_field_format);
        let previous_velocity_field = gpu::Texture2D::allocate(&context.context, dimensions, &velocity_field_format);

        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let density_field_format = gpu::TextureFormat::new(color_format, component_type);
        let density_field = gpu::Texture2D::allocate(&context.context, dimensions, &density_field_format);
        let previous_density_field = gpu::Texture2D::allocate(&context.context, dimensions, &density_field_format);

        Self { velocity_field, previous_velocity_field, density_field, previous_density_field, diffusion, viscosity, dimensions }
    }

    pub fn inner_volume(&self) -> f32 {
        ((self.dimensions.0 - 2) * (self.dimensions.1 - 2)) as f32
    }
}