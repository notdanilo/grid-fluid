/*
https://developer.download.nvidia.com/books/HTML/gpugems/gpugems_ch38.html
http://jamie-wong.com/2016/08/05/webgl-fluid-simulation/
https://29a.ch/2012/12/16/webgl-fluid-simulation
https://github.com/jwagner/fluidwebgl/blob/master/src/main.js
 */

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

    pub fn solve(&self, is_velocity_field: bool, current_field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, a: f32, c: f32, iterations: usize) {
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
            // self.boundary_limiter.limit(current_field, is_velocity_field);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;
    use crate::simulator::linear_solver::LinearSolver;
    use crate::initializer::Initializer;

    fn initialize(dimensions: (usize, usize)) -> (Context, LinearSolver, gpu::Texture2D, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let linear_solver = LinearSolver::new(&context);
        let (field, previous_field) = initialize_fields(&context);
        (context, linear_solver, field, previous_field)
    }

    fn initialize_vector(dimensions: (usize, usize)) -> (Context, LinearSolver, gpu::Texture2D, gpu::Texture2D) {
        let context = Context::new(dimensions);
        let linear_solver = LinearSolver::new(&context);
        let (field, previous_field) = initialize_vector_fields(&context);
        (context, linear_solver, field, previous_field)
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
    fn linear_solve() {
        let dimensions = (5, 5);
        let (context, linear_solver, mut field, previous_field) = initialize(dimensions);

        linear_solver.solve(false, )
    }
}