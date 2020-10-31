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
    pub fn new(context: &Context) -> Self {
        let diffuser = Diffuser::new(context);
        let advector = Advector::new(context);
        let projector = Projector::new(context);
        Self { diffuser, advector, projector }
    }

    pub fn simulate(&mut self, fluid: &mut Fluid, delta_time: f32) {
        let iterations = 4;
        // diffuse previous_velocity.xyz, velocity.xyz
        // diffuse previous_density.xyz, density.xyz
        // project previous_velocity.xyz, velocity.xy
        // advect velocity.xyz, previous_velocity.xyz
        // project velocity.xyz, previous_velocity.xy
        // advect density, previous_density, velocity.xyz
        self.diffuser.diffuse(fluid, delta_time, iterations);
        //self.projector.
    }
}