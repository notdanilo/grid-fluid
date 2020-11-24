mod divergence;
mod gradient;

use crate::context::Context;
use crate::simulator::boundary_limiter::BoundaryLimiter;
use crate::simulator::linear_solver::LinearSolver;
use divergence::Divergence;
use gradient::Gradient;

pub struct Projector {
    initialize_program : gpu::ComputeProgram,
    velocity_program: gpu::ComputeProgram,
    previous_velocity_program : gpu::ComputeProgram,
    div_field: gpu::Texture2D,
    p_field: gpu::Texture2D,
    boundary_limiter: BoundaryLimiter,
    linear_solver: LinearSolver,
    gradient_program: Gradient,
    divergence_program: Divergence
}

impl Projector {
    pub fn new(context: &Context, fluid_dimensions: (usize,usize)) -> Self {
        let initialize_shader = gpu::ComputeShader::new(&context.context, include_str!("initialize.glsl")).expect("Couldn't create initialize_shader.");
        let initialize_program = gpu::ComputeProgram::new(&context.context, &initialize_shader).expect("Couldn't create initialize_program.");

        let velocity_shader  = gpu::ComputeShader::new(&context.context, include_str!("velocity.glsl")).expect("Coudln't create velocity_shader.");
        let velocity_program = gpu::ComputeProgram::new(&context.context, &velocity_shader).expect("Couldn't create velocity_program.");

        let previous_velocity_shader  = gpu::ComputeShader::new(&context.context, include_str!("previous_velocity.glsl")).expect("Couldn't create previous_velocity_shader.");
        let previous_velocity_program = gpu::ComputeProgram::new(&context.context, &previous_velocity_shader).expect("Couldn't create previous_velocity_program.");

        let color_format   = gpu::ColorFormat::R;
        let component_type = gpu::Type::F32;
        let texture_format = gpu::TextureFormat::new(color_format, component_type);
        let div_field = gpu::Texture2D::allocate(&context.context, fluid_dimensions, &texture_format);
        let p_field   = gpu::Texture2D::allocate(&context.context, fluid_dimensions, &texture_format);

        let boundary_limiter = BoundaryLimiter::new(context);
        let linear_solver = LinearSolver::new(context, fluid_dimensions);

        let gradient_program = Gradient::new();
        let divergence_program = Divergence::new();
        Self { initialize_program, gradient_program, divergence_program, div_field, p_field, boundary_limiter, linear_solver, velocity_program, previous_velocity_program }
    }

    pub fn project(&mut self, velocity_field: &mut gpu::Texture2D, previous_velocity_field: &mut gpu::Texture2D, iterations: usize) {
        const VELOCITY_FIELD_LOCATION : usize = 0;
        const P_FIELD_LOCATION        : usize = 1;
        const DIV_FIELD_LOCATION      : usize = 2;
        const OFFSET_LOCATION         : usize = 2;
        self.initialize_program.bind_image_2d(velocity_field, VELOCITY_FIELD_LOCATION);
        self.initialize_program.bind_image_2d(&self.div_field, DIV_FIELD_LOCATION);
        self.initialize_program.bind_image_2d(&self.p_field, P_FIELD_LOCATION);
        let dimensions = velocity_field.dimensions();
        let dimensions = (dimensions.0, dimensions.1, 1);
        self.initialize_program.compute(dimensions);

        // self.boundary_limiter.limit(&mut self.div_field, false);
        // self.boundary_limiter.limit(&mut self.p_field, false);
        self.linear_solver.solve(false, &mut self.p_field, &self.div_field, 1.0, 6.0, iterations);

        let offset = (1, 1);
        self.velocity_program.bind_image_2d(velocity_field, VELOCITY_FIELD_LOCATION);
        self.velocity_program.bind_image_2d(&self.p_field, P_FIELD_LOCATION);
        self.velocity_program.bind_ivec2(offset, OFFSET_LOCATION);
        self.velocity_program.compute((dimensions.0 - 2, dimensions.1 - 2, 1));

        // self.boundary_limiter.limit(velocity_field, true);

        self.previous_velocity_program.bind_image_2d(previous_velocity_field, VELOCITY_FIELD_LOCATION);
        self.previous_velocity_program.bind_image_2d(&self.div_field, DIV_FIELD_LOCATION);
        self.previous_velocity_program.bind_image_2d(&self.p_field, P_FIELD_LOCATION);
        self.previous_velocity_program.compute(dimensions);
    }
}