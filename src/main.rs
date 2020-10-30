mod context;
mod fluid;
mod initializer;
mod presenter;
mod simulator;

use context::Context;
use initializer::Initializer;
use presenter::Presenter;
use simulator::Simulator;
use crate::fluid::Fluid;

fn main() {
    let dimensions = (512, 512);
    let mut context = Context::new(dimensions);

    let mut initializer = Initializer::new(&context);
    let mut presenter = Presenter::new(&context);
    let mut simulator = Simulator::new(&context);

    let diffusion = 0.0;
    let viscosity = 0.0;
    let mut fluid = Fluid::new(&context, dimensions, diffusion, viscosity);

    initializer.initialize(&context, &mut fluid);

    while context.context.run() {
        let delta_time = 0.1;
        simulator.simulate(&mut fluid, delta_time);
        presenter.present(&context, &fluid);
    }
}
