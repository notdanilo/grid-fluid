use crate::Context;

pub struct Fluid {
    pub framebuffer: gpu::Framebuffer,
    pub sampler: gpu::Sampler,
    pub viscosity: f32,
    pub diffusion: f32,
    pub delta_time: f32
}

impl Fluid {
    pub fn new(context: &Context, dimensions: (usize, usize), delta_time: f32, diffusion: f32, viscosity: f32) -> Self {
        let color_format = gpu::ColorFormat::RGBA;
        let component_type = gpu::Type::F32;
        let format  = gpu::TextureFormat::new(color_format, component_type);
        let texture = gpu::Texture2D::allocate(&context.context, dimensions, &format);
        let framebuffer = gpu::Framebuffer::new(&context.context, Some(texture), None, None).expect("Couldn't create Framebuffer.");
        let sampler = gpu::Sampler::new(&context.context);
        Self { delta_time, framebuffer, sampler, diffusion, viscosity }
    }
}