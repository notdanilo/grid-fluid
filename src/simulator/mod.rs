use crate::context::Context;
use crate::fluid::Fluid;

mod diffuser;
mod advector;
mod projector;
mod linear_solver;
mod boundary_limiter;

use diffuser::Diffuser;
use advector::Advector;
use projector::Projector;
use linear_solver::LinearSolver;
use boundary_limiter::BoundaryLimiter;

pub struct Simulator {
    diffuser: Diffuser,
    advector: Advector,
    projector: Projector
}

impl Simulator {
    pub fn new(context: &Context, dimensions: (usize, usize)) -> Self {
        let diffuser = Diffuser::new(context);
        let advector = Advector::new(context);
        let projector = Projector::new(context, dimensions);
        Self { diffuser, advector, projector }
    }

    pub fn simulate(&mut self, fluid: &mut Fluid, delta_time: f32) {
        // diffuse previous_velocity.xyz, velocity.xyz
        // diffuse previous_density.xyz, density.xyz
        // project previous_velocity.xyz, velocity.xy
        // advect velocity.xyz, previous_velocity.xyz, previous_velocity.xyz
        // project velocity.xyz, previous_velocity.xy
        // advect density, previous_density, velocity.xyz
        let iterations = 4;
        // self.diffuser.diffuse(fluid.diffusion, true, &mut fluid.previous_velocity_field, &fluid.velocity_field, delta_time, iterations);
        self.diffuser.diffuse(fluid.diffusion, false, &mut fluid.density_field, &fluid.previous_density_field, delta_time, iterations);
        // self.projector.project(&mut fluid.previous_velocity_field, &mut fluid.velocity_field, iterations);
        // self.advector.advect(true, &mut fluid.velocity_field, &fluid.previous_velocity_field, &fluid.previous_velocity_field, delta_time);
        // self.projector.project(&mut fluid.velocity_field, &mut fluid.previous_velocity_field, iterations);
        // self.advector.advect(false, &mut fluid.density_field, &fluid.previous_density_field, &fluid.velocity_field, delta_time);
    }
}