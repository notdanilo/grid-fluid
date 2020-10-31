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

    pub fn limit(&self, field: &mut gpu::Texture2D) {
        const FIELD_LOCATION       : usize = 0;
        const OFFSET_LOCATION      : usize = 1;
        const SIDE_NORMAL_LOCATION : usize = 2;
        let offset     = 1;
        let dimensions = field.dimensions();
        self.side_program.bind_image_2d(field, FIELD_LOCATION);
        self.side_program.bind_i32(offset, OFFSET_LOCATION);
        self.side_program.bind_ivec2((0, 1), SIDE_NORMAL_LOCATION);
        self.side_program.compute((dimensions.0, 2, 1));
        self.side_program.bind_ivec2((1, 0), SIDE_NORMAL_LOCATION);
        self.side_program.compute((2, dimensions.1, 1));
        let dimensions = (2, 2, 1);
        self.corner_program.program.bind_image_2d(field, FIELD_LOCATION);
        self.corner_program.compute(dimensions);
    }
}