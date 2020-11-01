use crate::context::Context;
use crate::simulator::boundary_limiter::BoundaryLimiter;

pub struct Advector {
    compute_program: gpu::ComputeProgram,
    boundary_limiter: BoundaryLimiter
}

impl Advector {
    pub fn new(context: &Context) -> Self {
        let compute_shader  = gpu::ComputeShader::new(&context.context, include_str!("compute.glsl")).expect("Couldn't create compute_shader.");
        let compute_program = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create compute_program.");

        let boundary_limiter = BoundaryLimiter::new(context);
        Self { compute_program, boundary_limiter }
    }

    pub fn advect(&self, is_velocity_field: bool, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        const OFFSET_LOCATION         : usize = 0;
        const FIELD_LOCATION          : usize = 1;
        const PREVIOUS_FIELD_LOCATION : usize = 2;
        const VECTOR_FIELD_LOCATION   : usize = 3;
        const DELTA_TIME_LOCATION     : usize = 4;
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0 - 2, dimensions.1 - 2, 1);
        let offset = (1, 1);
        self.compute_program.bind_ivec2(offset, OFFSET_LOCATION);
        self.compute_program.bind_image_2d(field, FIELD_LOCATION);
        self.compute_program.bind_image_2d(previous_field, PREVIOUS_FIELD_LOCATION);
        self.compute_program.bind_image_2d(velocity_field, VECTOR_FIELD_LOCATION);
        self.compute_program.bind_f32(delta_time, DELTA_TIME_LOCATION);
        self.compute_program.compute(dimensions);

        self.boundary_limiter.limit(field, is_velocity_field);
    }
}