use crate::context::Context;
use crate::fluid::Fluid;

pub struct Interactor {
    compute_program : gpu::ComputeProgram
}

impl Interactor {
    pub fn new(context: &Context) -> Self {
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("compute.glsl")).expect("Couldn't create compute_shader.");
        let compute_program = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create compute_program.");
        Self { compute_program }
    }

    pub fn interact(&self, fluid: &mut Fluid) {
        const VELOCITY_FIELD_LOCATION : usize = 0;
        const DENSITY_FIELD_LOCATION  : usize = 1;
        const POSITION_LOCATION       : usize = 2;
        let position   = (fluid.dimensions.0 as i32 / 2, fluid.dimensions.1 as i32 / 2);
        let dimensions = (1, 1, 1);
        self.compute_program.bind_image_2d(&fluid.velocity_field, VELOCITY_FIELD_LOCATION);
        self.compute_program.bind_image_2d(&fluid.density_field, DENSITY_FIELD_LOCATION);
        self.compute_program.bind_ivec2(position, POSITION_LOCATION);
        self.compute_program.compute(dimensions);
    }
}