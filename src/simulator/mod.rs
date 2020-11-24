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
        let diffuser = Diffuser::new(context, dimensions);
        let advector = Advector::new(context);
        let projector = Projector::new(context, dimensions);
        Self { diffuser, advector, projector }
    }

    pub fn simulate(&mut self, fluid: &mut Fluid, delta_time: f32) {
        std::mem::swap(&mut fluid.density_field, &mut fluid.previous_density_field);
        std::mem::swap(&mut fluid.velocity_field, &mut fluid.previous_velocity_field);
        let iterations = 30;
        // self.diffuser.diffuse(fluid.viscosity, true, &mut fluid.previous_velocity_field, &fluid.velocity_field, delta_time, iterations);
        // self.projector.project(&mut fluid.previous_velocity_field, &mut fluid.velocity_field, iterations);
        // self.advector.advect_vector_with_boundaries(true, &mut fluid.velocity_field, &fluid.previous_velocity_field, &fluid.previous_velocity_field, delta_time);
        // self.projector.project(&mut fluid.velocity_field, &mut fluid.previous_velocity_field, iterations);
        // self.diffuser.diffuse(fluid.diffusion, false, &mut fluid.previous_density_field, &fluid.density_field, delta_time, iterations);
        //self.advector.advect_vector_with_boundaries(true, &mut fluid.velocity_field, &fluid.previous_velocity_field, &fluid.previous_velocity_field, delta_time);
        self.advector.advect_vector(&mut fluid.velocity_field, &fluid.previous_velocity_field, &fluid.previous_velocity_field, delta_time);
        self.advector.advect_scalar(&mut fluid.density_field, &fluid.previous_density_field, &fluid.velocity_field, delta_time);
    }
}