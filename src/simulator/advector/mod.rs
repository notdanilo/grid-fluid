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
        const OFFSET_LOCATION         : usize = 4;
        let dimensions = field.dimensions();
        let dimensions = (dimensions.0 - 2, dimensions.1 - 2, 1);
        let offset = (1, 1);
        program.bind_image_2d(field, FIELD_LOCATION);
        program.bind_image_2d(previous_field, PREVIOUS_FIELD_LOCATION);
        program.bind_image_2d(velocity_field, VELOCITY_FIELD_LOCATION);
        program.bind_f32(delta_time, DELTA_TIME_LOCATION);
        program.bind_ivec2(offset, OFFSET_LOCATION);
        program.compute(dimensions);
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

    fn initialize(dimensions: (usize, usize)) -> (Context, Advector, gpu::Texture2D, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let advector = Advector::new(&context);
        let (field, previous_field) = initialize_fields(&context);
        (context, advector, field, previous_field)
    }

    fn initialize_vector(dimensions: (usize, usize)) -> (Context, Advector, gpu::Texture2D, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let advector = Advector::new(&context);
        let (field, previous_field) = initialize_vector_fields(&context);
        (context, advector, field, previous_field)
    }

    fn initialize_vector_fields(context: &Context) -> (gpu::Texture2D, gpu::Texture2D) {
        let dimensions = (5, 5);
        let data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        ];

        let previous_data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        ];

        let color_format = gpu::ColorFormat::RG;
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);

        let mut field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        let previous_field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &previous_data, &format);
        assert_eq!(field.data() as Vec<f32>, data);
        assert_eq!(previous_field.data() as Vec<f32>, previous_data);
        (field, previous_field)
    }

    fn initialize_fields(context: &Context) -> (gpu::Texture2D, gpu::Texture2D) {
        let dimensions = (5, 5);
        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let previous_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);

        let mut field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        let previous_field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &previous_data, &format);
        assert_eq!(field.data() as Vec<f32>, data);
        assert_eq!(previous_field.data() as Vec<f32>, previous_data);
        (field, previous_field)
    }

    #[test]
    fn zero_velocity_advection() {
        let dimensions = (5, 5);
        let (context, advector, mut field, previous_field) = initialize(dimensions);

        let velocity_color_format = gpu::ColorFormat::RG;
        let velocity_component_type = gpu::Type::F32;
        let velocity_format = gpu::TextureFormat::new(velocity_color_format, velocity_component_type);

        let zero_velocity_data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        ];

        let zero_velocity_field = gpu::Texture2D::from_data(&context.context, dimensions, &velocity_format, &zero_velocity_data, &velocity_format);
        assert_eq!(zero_velocity_field.data() as Vec<f32>, zero_velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &zero_velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &zero_velocity_field, 1.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);
    }

    #[test]
    fn discrete_advection() {
        let dimensions = (5, 5);
        let (context, advector, mut field, previous_field) = initialize(dimensions);

        let velocity_color_format = gpu::ColorFormat::RG;
        let velocity_component_type = gpu::Type::F32;
        let velocity_format = gpu::TextureFormat::new(velocity_color_format, velocity_component_type);

        let velocity_data = vec![
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 1.0, 1.0, /**/ 0.0, 0.0,
            0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0, /**/ 0.0, 0.0,
        ];

        let velocity_field = gpu::Texture2D::from_data(&context.context, dimensions, &velocity_format, &velocity_data, &velocity_format);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 1.0);
        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);
    }

    #[test]
    fn scalar_field_advection() {
        let dimensions = (5, 5);
        let (context, advector, mut field, previous_field) = initialize(dimensions);

        let velocity_color_format = gpu::ColorFormat::RG;
        let velocity_component_type = gpu::Type::F32;
        let velocity_format = gpu::TextureFormat::new(velocity_color_format, velocity_component_type);

        let velocity_data = vec![
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
        ];

        let velocity_field = gpu::Texture2D::from_data(&context.context, dimensions, &velocity_format, &velocity_data, &velocity_format);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.0);
        assert_eq!(field.data() as Vec<f32>, previous_field.data() as Vec<f32>);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.5);
        let data = vec![
            0.0, 0.0, 0.00, 0.00, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0,
            0.0, 0.0, 0.25, 0.25, 0.0,
            0.0, 0.0, 0.25, 0.25, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0
        ];
        assert_eq!(field.data() as Vec<f32>, data);

        advector.advect_scalar(&mut field, &previous_field, &velocity_field, 0.25);
        let data = vec![
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.0000, 0.0000, 0.0,
            0.0, 0.0, 0.5625, 0.1875, 0.0,
            0.0, 0.0, 0.1875, 0.0625, 0.0,
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
        let (context, advector, mut field, previous_field) = initialize_vector(dimensions);

        let velocity_color_format = gpu::ColorFormat::RG;
        let velocity_component_type = gpu::Type::F32;
        let velocity_format = gpu::TextureFormat::new(velocity_color_format, velocity_component_type);

        let velocity_data = vec![
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
            1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0, /**/ 1.0, 1.0,
        ];

        let velocity_field = gpu::Texture2D::from_data(&context.context, dimensions, &velocity_format, &velocity_data, &velocity_format);
        assert_eq!(velocity_field.data() as Vec<f32>, velocity_data);

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