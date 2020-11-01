use crate::context::Context;
use crate::fluid::Fluid;

pub struct BoundaryLimiter {
    corner_program: gpu::ComputeProgram,
    side_program: gpu::ComputeProgram
}

impl BoundaryLimiter {
    pub fn new(context: &Context) -> Self {
        let side_shader = gpu::ComputeShader::new(&context.context, include_str!("side.glsl")).expect("Couldn't create ComputeShader.");
        let side_program = gpu::ComputeProgram::new(&context.context, &side_shader).expect("Couldn't create ComputeProgram.");
        let corner_shader = gpu::ComputeShader::new(&context.context, include_str!("corner.glsl")).expect("Couldn't create ComputeShader.");
        let corner_program = gpu::ComputeProgram::new(&context.context, &corner_shader).expect("Couldn't create ComputeProgram.");
        Self { side_program, corner_program }
    }

    /// If it `is_velocity_field`, we reflect it on the corners.
    // FIXME: Remove is_velocity_field and move its functionality to a separate ComputeShader.
    // If we fix it, we need to be sure that it runs in the following order:
    // 1. side_program
    // 2. reflect_side_program
    // 3. corner_program
    pub fn limit(&self, field: &mut gpu::Texture2D, is_velocity_field: bool) {
        const FIELD_LOCATION             : usize = 0;
        const OFFSET_LOCATION            : usize = 1;
        const SIDE_NORMAL_LOCATION       : usize = 2;
        const IS_VELOCITY_FIELD_LOCATION : usize = 3;
        let offset     = 1;
        let dimensions = field.dimensions();
        self.side_program.bind_image_2d(field, FIELD_LOCATION);
        self.side_program.bind_i32(offset, OFFSET_LOCATION);
        self.side_program.bind_ivec2((0, 1), SIDE_NORMAL_LOCATION);
        self.side_program.compute((dimensions.0, 2, 1));
        self.side_program.bind_ivec2((1, 0), SIDE_NORMAL_LOCATION);
        self.side_program.bind_bool(is_velocity_field, IS_VELOCITY_FIELD_LOCATION);
        self.side_program.compute((2, dimensions.1, 1));
        let dimensions = (2, 2, 1);
        self.corner_program.bind_image_2d(field, FIELD_LOCATION);
        self.corner_program.compute(dimensions);
    }
}