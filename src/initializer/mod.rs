use crate::Context;
use crate::fluid::Fluid;

pub struct Initializer {
    pub scalar: gpu::ComputeProgram,
    pub vector: gpu::ComputeProgram
}

impl Initializer {
    pub fn new(context: &Context) -> Self {
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("initialize_scalar.glsl")).expect("Couldn't create ComputeShader.");
        let scalar = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create ComputeProgram.");
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("initialize_vector.glsl")).expect("Couldn't create ComputeShader.");
        let vector = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create ComputeProgram.");
        Self { scalar, vector }
    }

    pub fn initialize(&mut self, fluid: &mut Fluid) {
        self.initialize_scalar_field(&fluid.density_field);
        self.initialize_vector_field(&fluid.velocity_field);
        self.initialize_scalar_field(&fluid.previous_density_field);
        self.initialize_vector_field(&fluid.previous_velocity_field);
    }

    pub fn initialize_scalar_field(&mut self, field: &gpu::Texture2D) {
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0, dimensions.1, 1);
        self.scalar.bind_image_2d(field, 0);
        self.scalar.compute(dimensions);
        //FIXME: How to expose it on the GPU API?
        // Ref: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
        unsafe {
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }

    pub fn initialize_vector_field(&mut self, field: &gpu::Texture2D) {
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0, dimensions.1, 1);
        self.vector.bind_image_2d(field, 0);
        self.vector.compute(dimensions);
        //FIXME: How to expose it on the GPU API?
        // Ref: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
        unsafe {
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}