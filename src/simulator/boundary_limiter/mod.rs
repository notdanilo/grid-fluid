use crate::context::Context;
use crate::fluid::Fluid;

pub struct BoundaryLimiter {
    scalar_corner_program: gpu::ComputeProgram,
    scalar_side_program: gpu::ComputeProgram,
    vector_corner_program: gpu::ComputeProgram,
    vector_side_program: gpu::ComputeProgram,
}

impl BoundaryLimiter {
    pub fn new(context: &Context) -> Self {
        let side_shader = gpu::ComputeShader::new(&context.context, include_str!("scalar_side_2d.glsl")).expect("Couldn't create ComputeShader.");
        let scalar_side_program = gpu::ComputeProgram::new(&context.context, &side_shader).expect("Couldn't create ComputeProgram.");
        let corner_shader = gpu::ComputeShader::new(&context.context, include_str!("scalar_corner_2d.glsl")).expect("Couldn't create ComputeShader.");
        let scalar_corner_program = gpu::ComputeProgram::new(&context.context, &corner_shader).expect("Couldn't create ComputeProgram.");

        let side_shader = gpu::ComputeShader::new(&context.context, include_str!("vec2_side_2d.glsl")).expect("Couldn't create ComputeShader.");
        let vector_side_program = gpu::ComputeProgram::new(&context.context, &side_shader).expect("Couldn't create ComputeProgram.");
        let corner_shader = gpu::ComputeShader::new(&context.context, include_str!("vec2_corner_2d.glsl")).expect("Couldn't create ComputeShader.");
        let vector_corner_program = gpu::ComputeProgram::new(&context.context, &corner_shader).expect("Couldn't create ComputeProgram.");

        Self { scalar_side_program, scalar_corner_program, vector_corner_program, vector_side_program }
    }

    fn limit_sides_program(&self, program: &gpu::ComputeProgram, field: &mut gpu::Texture2D, is_velocity_field: bool) {
        const FIELD_LOCATION             : usize = 0;
        const OFFSET_LOCATION            : usize = 1;
        const SIDE_NORMAL_LOCATION       : usize = 2;
        const IS_VELOCITY_FIELD_LOCATION : usize = 3;
        let offset     = 1;
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0 - 2, dimensions.1 - 2);
        program.bind_bool(is_velocity_field, IS_VELOCITY_FIELD_LOCATION);
        program.bind_image_2d(field, FIELD_LOCATION);
        program.bind_i32(offset, OFFSET_LOCATION);
        program.bind_ivec2((0, 1), SIDE_NORMAL_LOCATION);
        program.compute((dimensions.0, 2, 1));
        program.bind_ivec2((1, 0), SIDE_NORMAL_LOCATION);
        program.compute((2, dimensions.1, 1));
    }

    fn limit_sides_scalar(&self, field: &mut gpu::Texture2D) {
        self.limit_sides_program(&self.scalar_side_program, field, false)
    }

    fn limit_sides_vector(&self, field: &mut gpu::Texture2D, is_velocity_field: bool) {
        self.limit_sides_program(&self.vector_side_program, field, is_velocity_field)
    }

    fn limit_corners_program(&self, program: &gpu::ComputeProgram, field: &mut gpu::Texture2D) {
        const FIELD_LOCATION             : usize = 0;
        let dimensions = (2, 2, 1);
        program.bind_image_2d(field, FIELD_LOCATION);
        program.compute(dimensions);
    }

    fn limit_corners_scalar(&self, field: &mut gpu::Texture2D) {
        self.limit_corners_program(&self.scalar_corner_program, field)
    }

    fn limit_corners_vector(&self, field: &mut gpu::Texture2D) {
        self.limit_corners_program(&self.vector_corner_program, field)
    }

    /// If it `is_velocity_field`, we reflect it on the corners.
    // FIXME: Remove is_velocity_field and move its functionality to a separate ComputeShader.
    // If we fix it, we need to be sure that it runs in the following order:
    // 1. side_program
    // 2. reflect_side_program
    // 3. corner_program

    pub fn limit_vector(&self, field: &mut gpu::Texture2D, is_velocity_field: bool) {
        self.limit_sides_vector(field, is_velocity_field);
        self.limit_corners_vector(field);
    }

    pub fn limit_scalar(&self, field: &mut gpu::Texture2D) {
        self.limit_sides_scalar(field);
        self.limit_corners_scalar(field);
    }
}

#[cfg(test)]
mod tests {
    use crate::simulator::boundary_limiter::BoundaryLimiter;
    use crate::context::Context;

    fn initialize(dimensions: (usize, usize), data: &Vec<f32>) -> (Context, BoundaryLimiter, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let limiter = BoundaryLimiter::new(&context);
        let field = initialize_fields(&context, data);
        (context, limiter, field)
    }

    fn initialize_fields(context: &Context, data: &Vec<f32>) -> gpu::Texture2D {
        let dimensions = (5, 5);

        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);

        let field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        assert_eq!(field.data() as Vec<f32>, *data);
        field
    }

    fn initialize_vector(dimensions: (usize, usize), data: &Vec<f32>) -> (Context, BoundaryLimiter, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let limiter = BoundaryLimiter::new(&context);
        let field = initialize_vector_fields(&context, data);
        (context, limiter, field)
    }

    fn initialize_vector_fields(context: &Context, data: &Vec<f32>) -> gpu::Texture2D {
        let dimensions = (5, 5);

        let color_format = gpu::ColorFormat::RG;
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);

        let field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        assert_eq!(field.data() as Vec<f32>, *data);
        field
    }

    #[test]
    fn scalar_sides() {
        let dimensions = (5, 5);

        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 2.0, 3.0, 0.0,
            0.0, 4.0, 5.0, 6.0, 0.0,
            0.0, 7.0, 8.0, 9.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let (_context, limiter, mut field) = initialize(dimensions, &data);

        limiter.limit_sides_scalar(&mut field);

        let expected_data = vec![
            0.0, 1.0, 2.0, 3.0, 0.0,
            1.0, 1.0, 2.0, 3.0, 3.0,
            4.0, 4.0, 5.0, 6.0, 6.0,
            7.0, 7.0, 8.0, 9.0, 9.0,
            0.0, 7.0, 8.0, 9.0, 0.0
        ];

        assert_eq!(field.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn scalar_corners() {
        let dimensions = (5, 5);

        let data = vec![
            0.0, 1.0, 2.0, 3.0, 0.0,
            1.0, 1.0, 2.0, 3.0, 3.0,
            4.0, 4.0, 5.0, 6.0, 6.0,
            7.0, 7.0, 8.0, 9.0, 9.0,
            0.0, 7.0, 8.0, 9.0, 0.0
        ];

        let (_context, limiter, mut field) = initialize(dimensions, &data);

        limiter.limit_corners_scalar(&mut field);

        let expected_data = vec![
            1.0, 1.0, 2.0, 3.0, 3.0,
            1.0, 1.0, 2.0, 3.0, 3.0,
            4.0, 4.0, 5.0, 6.0, 6.0,
            7.0, 7.0, 8.0, 9.0, 9.0,
            7.0, 7.0, 8.0, 9.0, 9.0
        ];

        assert_eq!(field.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn scalar_boundaries() {
        let dimensions = (5, 5);

        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 2.0, 3.0, 0.0,
            0.0, 4.0, 5.0, 6.0, 0.0,
            0.0, 7.0, 8.0, 9.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let (_context, limiter, mut field) = initialize(dimensions, &data);

        limiter.limit_scalar(&mut field);

        let expected_data = vec![
            1.0, 1.0, 2.0, 3.0, 3.0,
            1.0, 1.0, 2.0, 3.0, 3.0,
            4.0, 4.0, 5.0, 6.0, 6.0,
            7.0, 7.0, 8.0, 9.0, 9.0,
            7.0, 7.0, 8.0, 9.0, 9.0
        ];

        assert_eq!(field.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn vector_sides() {
        let dimensions = (5, 5);

        let data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0,  0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 4.0, 5.0, /**/ 5.0, 6.0, /**/ 6.0,  7.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0,  0.0, /**/ 0.0, 0.0,
        ];

        let (_context, limiter, mut field) = initialize_vector(dimensions, &data);

        limiter.limit_sides_vector(&mut field, false);

        let expected_data = vec![
            0.0, 0.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 0.0,  0.0,
            1.0, 2.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 3.0,  4.0,
            4.0, 5.0, /**/ 4.0, 5.0, /**/ 5.0, 6.0, /**/ 6.0,  7.0, /**/ 6.0,  7.0,
            7.0, 8.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 9.0, 10.0,
            0.0, 0.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 0.0,  0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, expected_data);

        limiter.limit_sides_vector(&mut field, true);

        let expected_data = vec![
             0.0, 0.0, /**/ 1.0, -2.0, /**/ 2.0, -3.0, /**/ 3.0,  -4.0, /**/  0.0,  0.0,
            -1.0, 2.0, /**/ 1.0,  2.0, /**/ 2.0,  3.0, /**/ 3.0,   4.0, /**/ -3.0,  4.0,
            -4.0, 5.0, /**/ 4.0,  5.0, /**/ 5.0,  6.0, /**/ 6.0,   7.0, /**/ -6.0,  7.0,
            -7.0, 8.0, /**/ 7.0,  8.0, /**/ 8.0,  9.0, /**/ 9.0,  10.0, /**/ -9.0, 10.0,
             0.0, 0.0, /**/ 7.0, -8.0, /**/ 8.0, -9.0, /**/ 9.0, -10.0, /**/  0.0,  0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn vector_mirrored_corners() {
        let dimensions = (5, 5);

        let data = vec![
              0.0, 0.0, /**/ 1.0, -2.0, /**/ 2.0, -3.0, /**/ 3.0,  -4.0, /**/  0.0,  0.0,
             -1.0, 2.0, /**/ 1.0,  2.0, /**/ 2.0,  3.0, /**/ 3.0,   4.0, /**/ -3.0,  4.0,
             -4.0, 5.0, /**/ 4.0,  5.0, /**/ 5.0,  6.0, /**/ 6.0,   7.0, /**/ -6.0,  7.0,
             -7.0, 8.0, /**/ 7.0,  8.0, /**/ 8.0,  9.0, /**/ 9.0,  10.0, /**/ -9.0, 10.0,
              0.0, 0.0, /**/ 7.0, -8.0, /**/ 8.0, -9.0, /**/ 9.0, -10.0, /**/  0.0,  0.0,
        ];

        let (_context, limiter, mut field) = initialize_vector(dimensions, &data);

        limiter.limit_corners_vector(&mut field);

        let expected_data = vec![
             0.0, 0.0, /**/ 1.0, -2.0, /**/ 2.0, -3.0, /**/ 3.0,  -4.0, /**/  0.0,  0.0,
            -1.0, 2.0, /**/ 1.0,  2.0, /**/ 2.0,  3.0, /**/ 3.0,   4.0, /**/ -3.0,  4.0,
            -4.0, 5.0, /**/ 4.0,  5.0, /**/ 5.0,  6.0, /**/ 6.0,   7.0, /**/ -6.0,  7.0,
            -7.0, 8.0, /**/ 7.0,  8.0, /**/ 8.0,  9.0, /**/ 9.0,  10.0, /**/ -9.0, 10.0,
             0.0, 0.0, /**/ 7.0, -8.0, /**/ 8.0, -9.0, /**/ 9.0, -10.0, /**/  0.0,  0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn vector_corners() {
        let dimensions = (5, 5);

        let data = vec![
            0.0, 0.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 0.0,  0.0,
            1.0, 2.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 3.0,  4.0,
            4.0, 5.0, /**/ 4.0, 5.0, /**/ 5.0, 6.0, /**/ 6.0,  7.0, /**/ 6.0,  7.0,
            7.0, 8.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 9.0, 10.0,
            0.0, 0.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 0.0,  0.0,
        ];

        let (_context, limiter, mut field) = initialize_vector(dimensions, &data);

        limiter.limit_corners_vector(&mut field);

        let expected_data = vec![
            1.0, 2.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 3.0,  4.0,
            1.0, 2.0, /**/ 1.0, 2.0, /**/ 2.0, 3.0, /**/ 3.0,  4.0, /**/ 3.0,  4.0,
            4.0, 5.0, /**/ 4.0, 5.0, /**/ 5.0, 6.0, /**/ 6.0,  7.0, /**/ 6.0,  7.0,
            7.0, 8.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 9.0, 10.0,
            7.0, 8.0, /**/ 7.0, 8.0, /**/ 8.0, 9.0, /**/ 9.0, 10.0, /**/ 9.0, 10.0,
        ];
        assert_eq!(field.data() as Vec<f32>, expected_data);
    }
}