use crate::context::Context;
use crate::fluid::Fluid;
use crate::simulator::boundary_limiter::BoundaryLimiter;

pub struct LinearSolver {
    boundary_limiter: BoundaryLimiter,
    compute_program: gpu::ComputeProgram
}

impl LinearSolver {
    pub fn new(context: &Context) -> Self {
        let boundary_limiter = BoundaryLimiter::new(context);
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("compute.glsl")).expect("Couldn't create ComputeShader.");
        let compute_program = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create ComputeProgram.");
        Self { boundary_limiter, compute_program }
    }

    pub fn solve(&self, current_field: &mut gpu::Texture2D, previous_field: &mut gpu::Texture2D, a: f32, c: f32, iterations: usize) {
        let c_reciprocal = 1.0 / c;
        let offset = (1, 1);

        const CURRENT_FIELD_LOCATION  : usize = 0;
        const PREVIOUS_FIELD_LOCATION : usize = 1;
        const OFFSET_LOCATION         : usize = 2;
        const A_LOCATION              : usize = 3;
        const C_RECIPROCAL_LOCATION   : usize = 4;

        let dimensions = current_field.dimensions();
        let dimensions = (dimensions.0 - 2, dimensions.1 - 2, 1);
        for _ in 0 .. iterations {
            self.compute_program.bind_ivec2(offset, OFFSET_LOCATION);
            self.compute_program.bind_f32(a, A_LOCATION);
            self.compute_program.bind_f32(c_reciprocal, C_RECIPROCAL_LOCATION);
            self.compute_program.bind_image_2d(current_field, CURRENT_FIELD_LOCATION);
            self.compute_program.bind_image_2d(previous_field, PREVIOUS_FIELD_LOCATION);
            self.compute_program.compute(dimensions);
            self.boundary_limiter.limit(current_field);
        }
    }
}
