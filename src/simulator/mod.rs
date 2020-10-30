use crate::context::Context;
use crate::fluid::Fluid;

mod diffuser;
mod advector;
mod projector;

use diffuser::Diffuser;
use advector::Advector;
use projector::Projector;

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
        //self.diffuser.diffuse(&mut fluid, delta_time);
    }
}