mod context;
mod fluid;
mod initializer;
mod presenter;
mod simulator;
mod interactor;

use context::Context;
use initializer::Initializer;
use presenter::Presenter;
use simulator::Simulator;
use fluid::Fluid;
use interactor::Interactor;

fn main() {
    let dimensions = (512, 512);
    let mut context = Context::new(dimensions);

    let mut initializer = Initializer::new(&context);
    let mut presenter = Presenter::new(&context);
    let mut simulator = Simulator::new(&context, dimensions);
    let interactor = Interactor::new(&context);

    let diffusion = 0.0;
    let viscosity = 0.0000001;
    let mut fluid = Fluid::new(&context, dimensions, diffusion, viscosity);

    initializer.initialize(&context, &mut fluid);

    let delta_time = 0.2;
    while context.context.run() {
        interactor.interact(&mut fluid);
        simulator.simulate(&mut fluid, delta_time);
        presenter.present(&context, &fluid);
    }
}
