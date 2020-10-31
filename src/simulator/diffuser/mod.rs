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

    pub fn diffuse(&self, fluid: &mut Fluid, delta_time: f32, iterations: usize) {
        let a = delta_time * fluid.diffusion * fluid.inner_volume();
        let c = 1.0 + 6.0 * a;
        self.linear_solver.solve(&mut fluid.previous_velocity, &mut fluid.velocity        , a, c, iterations);
        self.linear_solver.solve(&mut fluid.density          , &mut fluid.previous_density, a, c, iterations);
    }
}