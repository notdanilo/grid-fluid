use crate::context::Context;
use crate::fluid::Fluid;
use crate::simulator::linear_solver::LinearSolver;

pub struct Diffuser {
    linear_solver: LinearSolver
}

impl Diffuser {
    pub fn new(context: &Context) -> Self {
        let linear_solver = LinearSolver::new(context);
        Self { linear_solver }
    }

    pub fn diffuse(&self, diffusion: f32, is_velocity_field: bool, current_field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, delta_time: f32, iterations: usize) {
        let dimensions = current_field.dimensions();
        let dimensions = (dimensions.0 - 2, dimensions.1 - 2); // inner volume without the borders.
        let volume = (dimensions.0 * dimensions.1) as f32;
        let a = delta_time * diffusion * volume;
        let c = 1.0 + 6.0 * a;
        self.linear_solver.solve(is_velocity_field, current_field, previous_field, a, c, iterations);
    }
}