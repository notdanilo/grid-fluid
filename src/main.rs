mod context;
mod fluid;
mod initializer;
mod presenter;

use context::Context;
use initializer::Initializer;
use presenter::Presenter;
use crate::fluid::Fluid;

fn main() {
    let dimensions = (512, 512);
    let mut context = Context::new(dimensions);

    let mut initializer = Initializer::new(&context);
    let mut presenter = Presenter::new(&context);

    let delta_time = 0.1;
    let diffusion = 0.0;
    let viscosity = 0.0;
    let fluid = Fluid::new(&context, dimensions, delta_time, diffusion, viscosity);

    initializer.initialize(&context, &fluid);

    while context.context.run() {
        presenter.present(&context, &fluid);
    }
}
