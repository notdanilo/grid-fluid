mod field;
mod context;
mod fluid;
mod initializer;
mod presenter;
mod simulator;
mod interactor;
mod velocity_debugger;

use field::Field;
use context::Context;
use initializer::Initializer;
use presenter::Presenter;
use simulator::Simulator;
use fluid::Fluid;
use interactor::Interactor;
use velocity_debugger::VelocityDebugger;
use std::time::Instant;

fn main() {
    let dimensions = (256, 256);
    let mut context = Context::new(dimensions);

    let mut initializer = Initializer::new(&context);
    let mut presenter = Presenter::new(&context);
    let mut simulator = Simulator::new(&context, dimensions);
    let interactor = Interactor::new(&context);
    let velocity_debugger = VelocityDebugger::new(&context);

    let diffusion = 1.0;
    let viscosity = 0.0000001;
    let mut fluid = Fluid::new(&context, dimensions, diffusion, viscosity);

    initializer.initialize(&mut fluid);

    let delta_time = 0.001;
    let mut then = Instant::now();
    while context.context.run() {
        let now = Instant::now();
        let delta_time = (now - then).as_secs_f32();
        then = now;
        // interactor.interact(&mut fluid);
        simulator.simulate(&mut fluid, delta_time);
        presenter.present(&context, &fluid);
        velocity_debugger.debug(&fluid.velocity_field);
        context.present();
    }
}
