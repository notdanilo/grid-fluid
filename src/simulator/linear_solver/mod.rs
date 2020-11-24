/*
https://developer.download.nvidia.com/books/HTML/gpugems/gpugems_ch38.html
http://jamie-wong.com/2016/08/05/webgl-fluid-simulation/
https://29a.ch/2012/12/16/webgl-fluid-simulation
https://github.com/jwagner/fluidwebgl/blob/master/src/main.js
 */

use crate::context::Context;
use crate::fluid::Fluid;
use crate::simulator::boundary_limiter::BoundaryLimiter;
use crate::initializer::Initializer;

pub struct LinearSolver {
    boundary_limiter: BoundaryLimiter,
    compute_program: gpu::ComputeProgram,
    temporary_x_scalar: gpu::Texture2D,
    temporary_x_vector: gpu::Texture2D
}

impl LinearSolver {
    const OUTPUT_FIELD_LOCATION    : usize = 0;
    const X_FIELD_LOCATION         : usize = 1;
    const B_FIELD_LOCATION         : usize = 2;
    const ALPHA_LOCATION           : usize = 3;
    const RECIPROCAL_BETA_LOCATION : usize = 4;
    const OFFSET_LOCATION          : usize = 5;

    pub fn new(context: &Context, dimensions:(usize, usize)) -> Self {
        let boundary_limiter = BoundaryLimiter::new(context);
        let compute_shader = gpu::ComputeShader::new(&context.context, include_str!("compute.glsl")).expect("Couldn't create ComputeShader.");
        let compute_program = gpu::ComputeProgram::new(&context.context, &compute_shader).expect("Couldn't create ComputeProgram.");

        let component_type = gpu::Type::F32;
        let x_scalar_components = gpu::ColorFormat::R;
        let x_scalar_format = gpu::TextureFormat::new(x_scalar_components, component_type);
        let temporary_x_scalar = gpu::Texture2D::allocate(&context.context, dimensions, &x_scalar_format);
        let x_vector_components = gpu::ColorFormat::RG;
        let x_vector_format = gpu::TextureFormat::new(x_vector_components, component_type);
        let temporary_x_vector = gpu::Texture2D::allocate(&context.context, dimensions, &x_vector_format);

        let mut initialize = Initializer::new(context);
        initialize.initialize_vector_field(&temporary_x_vector);
        initialize.initialize_scalar_field(&temporary_x_scalar);

        Self { boundary_limiter, compute_program, temporary_x_scalar, temporary_x_vector }
    }

    fn upload(&self, is_velocity_field: bool, x: &mut gpu::Texture2D, b: &gpu::Texture2D, alpha: f32, beta: f32) -> (usize, usize, usize) {
        let reciprocal_beta = 1.0 / beta;
        let offset = (0, 0);

        self.compute_program.bind_ivec2(offset       , Self::OFFSET_LOCATION);
        self.compute_program.bind_f32(alpha          , Self::ALPHA_LOCATION);
        self.compute_program.bind_f32(reciprocal_beta, Self::RECIPROCAL_BETA_LOCATION);
        self.compute_program.bind_image_2d(b         , Self::B_FIELD_LOCATION);

        let dimensions = x.dimensions();
        (dimensions.0, dimensions.1, 1)
    }

    pub fn solve(&mut self, is_velocity_field: bool, x: &mut gpu::Texture2D, b: &gpu::Texture2D, alpha: f32, beta: f32, iterations: usize) {
        let dimensions = self.upload(is_velocity_field, x, b, alpha, beta);
        for _ in 0 .. iterations {
            self.compute_program.bind_image_2d(&mut self.temporary_x_scalar, Self::OUTPUT_FIELD_LOCATION);
            self.compute_program.bind_image_2d(x, Self::X_FIELD_LOCATION);
            self.compute_program.compute(dimensions);
            //FIXME: How to expose it on the GPU API?
            // Ref: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
            unsafe {
                gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            }
            std::mem::swap(&mut self.temporary_x_scalar, x);
        }
    }

    pub fn solve_with_boundaries(&mut self, is_velocity_field: bool, x: &mut gpu::Texture2D, b: &gpu::Texture2D, alpha: f32, beta: f32, iterations: usize) {
        let dimensions = self.upload(is_velocity_field, x, b, alpha, beta);
        for _ in 0 .. iterations {
            self.compute_program.bind_image_2d(&mut self.temporary_x_scalar, Self::OUTPUT_FIELD_LOCATION);
            self.compute_program.bind_image_2d(x, Self::X_FIELD_LOCATION);
            self.compute_program.compute(dimensions);
            // self.boundary_limiter.limit(current_field, is_velocity_field);
            std::mem::swap(&mut self.temporary_x_scalar, x);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;
    use crate::simulator::linear_solver::LinearSolver;
    use crate::initializer::Initializer;

    fn initialize(dimensions: (usize, usize)) -> (Context, LinearSolver) {
        let context = Context::new(dimensions);
        let linear_solver = LinearSolver::new(&context, dimensions);
        (context, linear_solver)
    }

    fn initialize_field(context: &Context, dimensions:(usize, usize), data: &[f32]) -> gpu::Texture2D {
        let color_format = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let format = gpu::TextureFormat::new(color_format, component_type);

        let field = gpu::Texture2D::from_data(&context.context, dimensions, &format, &data, &format);
        assert_eq!(field.data() as Vec<f32>, data);
        field
    }

    #[test]
    fn copy_b_to_x() {
        let dimensions = (5, 5);
        let (context, mut linear_solver) = initialize(dimensions);

        let x_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let b_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let mut x = initialize_field(&context, dimensions, &x_data);
        let b = initialize_field(&context, dimensions, &b_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 1);

        assert_eq!(x.data() as Vec<f32>, b_data);
    }

    #[test]
    fn copy_half_b_to_x() {
        let dimensions = (5, 5);
        let (context, mut linear_solver) = initialize(dimensions);

        let x_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let b_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let mut x = initialize_field(&context, dimensions, &x_data);
        let b = initialize_field(&context, dimensions, &b_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 2.0, 1);

        let expected_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.5, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 0.0, 0.0,
            0.0, 0.5, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        assert_eq!(x.data() as Vec<f32>, expected_data);

        let mut x = initialize_field(&context, dimensions, &x_data);
        linear_solver.solve(false, &mut x, &b, 0.5, 1.0, 1);
        assert_eq!(x.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn various() {
        let dimensions = (5, 5);
        let (context, mut linear_solver) = initialize(dimensions);

        let x_data = vec![
            1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0
        ];

        let b_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let mut x = initialize_field(&context, dimensions, &x_data);
        let b = initialize_field(&context, dimensions, &b_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 1);

        let expected_data = vec![
            2.0, 2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 1.0, 2.0, 2.0,
            2.0, 1.0, 0.0, 1.0, 2.0,
            2.0, 2.0, 1.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0, 2.0
        ];
        assert_eq!(x.data() as Vec<f32>, expected_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 1);
        let expected_data = vec![
            4.0, 6.0, 5.0, 6.0, 4.0,
            6.0, 6.0, 6.0, 6.0, 6.0,
            5.0, 6.0, 4.0, 6.0, 5.0,
            6.0, 6.0, 6.0, 6.0, 6.0,
            4.0, 6.0, 5.0, 6.0, 4.0
        ];
        assert_eq!(x.data() as Vec<f32>, expected_data);

        let mut x = initialize_field(&context, dimensions, &x_data);
        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 2);
        assert_eq!(x.data() as Vec<f32>, expected_data);


        let b_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let mut x = initialize_field(&context, dimensions, &x_data);
        let b = initialize_field(&context, dimensions, &b_data);
        linear_solver.solve(false, &mut x, &b, 2.0, 2.0, 1);
        let expected_data = vec![
            1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 0.5, 1.0, 1.0,
            1.0, 0.5, 1.0, 0.5, 1.0,
            1.0, 1.0, 0.5, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0
        ];
        assert_eq!(x.data() as Vec<f32>, expected_data);
    }

    #[test]
    fn spread_x() {
        let dimensions = (5, 5);
        let (context, mut linear_solver) = initialize(dimensions);

        let x_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let b_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];

        let mut x = initialize_field(&context, dimensions, &x_data);
        let b = initialize_field(&context, dimensions, &b_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 1);
        let expected_data = vec![
            0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0
        ];
        assert_eq!(x.data() as Vec<f32>, expected_data);

        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 1);
        let expected_data = vec![
            0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 2.0, 0.0,
            1.0, 0.0, 4.0, 0.0, 1.0,
            0.0, 2.0, 0.0, 2.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0
        ];
        assert_eq!(x.data() as Vec<f32>, expected_data);

        let mut x = initialize_field(&context, dimensions, &x_data);
        linear_solver.solve(false, &mut x, &b, 1.0, 1.0, 2);
        assert_eq!(x.data() as Vec<f32>, expected_data);
    }
}