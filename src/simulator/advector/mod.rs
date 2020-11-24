use crate::context::Context;
use crate::simulator::boundary_limiter::BoundaryLimiter;

pub struct Advector {
    scalar_advection_program: gpu::ComputeProgram,
    vector_advection_program: gpu::ComputeProgram,
    boundary_limiter: BoundaryLimiter
}

impl Advector {
    pub fn new(context: &Context) -> Self {
        let scalar_advection_shader  = gpu::ComputeShader::new(&context.context, include_str!("scalar_advection_2d.glsl")).expect("Couldn't create compute_shader.");
        let scalar_advection_program = gpu::ComputeProgram::new(&context.context, &scalar_advection_shader).expect("Couldn't create compute_program.");

        let vector_advection_shader  = gpu::ComputeShader::new(&context.context, include_str!("vec2_advection_2d.glsl")).expect("Couldn't create compute_shader.");
        let vector_advection_program = gpu::ComputeProgram::new(&context.context, &vector_advection_shader).expect("Couldn't create compute_program.");

        let boundary_limiter = BoundaryLimiter::new(context);
        Self { scalar_advection_program, vector_advection_program, boundary_limiter }
    }

    fn advect_program(&self, program: &gpu::ComputeProgram, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        const FIELD_LOCATION          : usize = 0;
        const PREVIOUS_FIELD_LOCATION : usize = 1;
        const VELOCITY_FIELD_LOCATION : usize = 2;
        const DELTA_TIME_LOCATION     : usize = 3;
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0, dimensions.1, 1);
        program.bind_image_2d(field, FIELD_LOCATION);
        program.bind_image_2d(previous_field, PREVIOUS_FIELD_LOCATION);
        program.bind_image_2d(velocity_field, VELOCITY_FIELD_LOCATION);
        program.bind_f32(delta_time, DELTA_TIME_LOCATION);
        program.compute(dimensions);
        //FIXME: How to expose it on the GPU API?
        // Ref: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
        unsafe {
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }

    pub fn advect_scalar(&self, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        self.advect_program(&self.scalar_advection_program, field, previous_field, velocity_field, delta_time)
    }

    pub fn advect_scalar_with_boundaries(&self, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        self.advect_scalar(field, previous_field, velocity_field, delta_time);
        self.boundary_limiter.limit_scalar(field);
    }

    pub fn advect_vector(&self, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        self.advect_program(&self.vector_advection_program, field, previous_field, velocity_field, delta_time)
    }

    pub fn advect_vector_with_boundaries(&self, is_velocity_field: bool, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D, delta_time: f32) {
        self.advect_vector(field, previous_field, velocity_field, delta_time);
        self.boundary_limiter.limit_vector(field, is_velocity_field);
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;
    use crate::simulator::advector::Advector;
    use crate::initializer::Initializer;

    fn initialize(dimensions: (usize, usize)) -> (Context, Advector) {
        let context = Context::new(dimensions);
        let advector = Advector::new(&context);
        (context, advector)
    }

    const ZERO_SCALAR_FIELD: [f32; 25] = [
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0
    ];

    const SCALAR_FIELD_CENTER: [f32; 25] = [
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0
    ];

    const ZERO_VECTOR_FIELD : [f32; 50] = [
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
    ];

    const VECTOR_FIELD_CENTER: [f32; 50] = [
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
    ];

    fn initialize_field(context: &Context, dimensions: (usize, usize), data: &[f32], color_format: gpu::ColorFormat) -> gpu::Texture2D {
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);
        let field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        assert_eq!(field.data() as Vec<f32>, data);
        field
    }

    fn initialize_scalar_field(context: &Context, dimensions: (usize, usize), data: &[f32]) -> gpu::Texture2D {
        initialize_field(context, dimensions, data, gpu::ColorFormat::R)
    }

    fn initialize_vector_field(context: &Context, dimensions: (usize, usize), data: &[f32]) -> gpu::Texture2D {
        initialize_field(context, dimensions, data, gpu::ColorFormat::RG)
    }

    #[test]
    fn zero_velocity_advection() {
        let dimensions = (5, 5);
        let (context, advector) = initialize(dimensions);

        let mut field = initialize_scalar_field(&context, dimensions, &ZERO_SCALAR_FIELD);
        let previous_field = initialize_scalar_field(&context, dimensions, &SCALAR_FIELD_CENTER);

        let zero_velocity_field = initialize_vector_field(&context, dimensions, &ZERO_VECTOR_FIELD);
        assert_eq!(zero_velocity_field.data() as Vec<f32>, ZERO_VECTOR_FIELD.to_vec());

        advector.advect_scalar(&mut field, &previous_field, &zero_velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &zero_velocity_field, 1.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);
    }


    #[test]
    fn discrete_advection_1() {
        let dimensions = (5, 5);
        let (context, advector) = initialize(dimensions);

        let mut field = initialize_scalar_field(&context, dimensions, &ZERO_SCALAR_FIELD);
        let previous_field = initialize_scalar_field(&context, dimensions, &[
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0
        ]);

        let velocity_data = vec![
            1.0, 1.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0,
        ];
        let velocity_field = initialize_vector_field(&context, dimensions, &velocity_data);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 1.0);
        let data = vec![
            1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);
    }

    #[test]
    fn discrete_advection_2() {
        let dimensions = (5, 5);
        let (context, advector) = initialize(dimensions);

        let mut field = initialize_scalar_field(&context, dimensions, &ZERO_SCALAR_FIELD);
        let previous_field = initialize_scalar_field(&context, dimensions, &[
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0
        ]);

        let velocity_data = vec![
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
        ];
        let velocity_field = initialize_vector_field(&context, dimensions, &velocity_data);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 1.0);
        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);
    }

    #[test]
    fn scalar_field_advection() {
        let dimensions = (5, 5);
        let (context, advector) = initialize(dimensions);

        let mut field = initialize_scalar_field(&context, dimensions, &ZERO_SCALAR_FIELD);
        let previous_field = initialize_scalar_field(&context, dimensions, &SCALAR_FIELD_CENTER);

        let velocity_data = vec![
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
            1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0, /**/ 1.0, -1.0,
        ];

        let velocity_field = initialize_vector_field(&context, dimensions, &velocity_data);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.5);
        let data = vec![
            0.0, 0.0, 0.00, 0.00, 0.0,
            0.0, 0.0, 0.25, 0.25, 0.0,
            0.0, 0.0, 0.25, 0.25, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.25);
        let data = vec![
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.1875, 0.0625, 0.0,
            0.0, 0.0, 0.5625, 0.1875, 0.0,
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.0000, 0.0000, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.75);
        let data = vec![
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.0625, 0.1875, 0.0,
            0.0, 0.0, 0.1875, 0.5625, 0.0,
            0.0, 0.0, 0.0000, 0.0000, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);
    }

    #[test]
    fn vector_field_advection() {
        let dimensions = (5, 5);
        let (context, advector) = initialize(dimensions);

        let mut field = initialize_vector_field(&context, dimensions, &ZERO_VECTOR_FIELD);
        let previous_field = initialize_vector_field(&context, dimensions, &VECTOR_FIELD_CENTER);

        let velocity_data = vec![
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
        ];

        let velocity_field = initialize_vector_field(&context, dimensions, &velocity_data);

        advector.advect_vector(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_vector(&mut field, &previous_field, &velocity_field, 0.5);
        let data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.00, 0.00, /**/ 0.00, 0.00, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.00, 0.00, /**/ 0.00, 0.00, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.25, 0.25, /**/ 0.25, 0.25, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.25, 0.25, /**/ 0.25, 0.25, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.00, 0.00, /**/ 0.00, 0.00, /**/ 0.0, 0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, data);

        advector.advect_vector(&mut field, &previous_field, &velocity_field, 0.25);
        let data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.5625, 0.5625, /**/ 0.1875, 0.1875, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.1875, 0.1875, /**/ 0.0625, 0.0625, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, data);

        advector.advect_vector(&mut field, &previous_field, &velocity_field, 0.75);
        let data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0625, 0.0625, /**/ 0.1875, 0.1875, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.1875, 0.1875, /**/ 0.5625, 0.5625, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0000, 0.0000, /**/ 0.0000, 0.0000, /**/ 0.0, 0.0,
        ];
        assert_eq!(field.data() as Vec<f32>, data);
    }
}
