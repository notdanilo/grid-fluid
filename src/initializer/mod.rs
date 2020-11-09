use crate::Context;
use crate::fluid::Fluid;

pub struct Initializer {
    pub compute_program: gpu::ComputeProgram
}

impl Initializer {
    pub fn new(context: &Context) -> Self {
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("scalar_advection.glsl")).expect("Couldn't create ComputeShader.");
        let compute_program = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create ComputeProgram.");
        Self { compute_program }
    }

    pub fn initialize(&mut self, context: &Context, fluid: &mut Fluid) {
        let dimensions = fluid.density_field.dimensions();
        let dimensions = (dimensions.0, dimensions.1, 1);
        self.compute_program.bind_image_2d(&fluid.velocity_field, 0);
        self.compute_program.bind_image_2d(&fluid.density_field, 1);
        self.compute_program.compute(dimensions);
    }
}