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

pub struct Field {
    pub data : Vec<f32>,
    dimensions: (usize, usize)
}

impl Field {
    pub fn new(data: Vec<f32>, dimensions:(usize,usize)) -> Self {
        Self { data, dimensions }
    }

    pub fn print_float(&self) {
        for i in 0 .. self.dimensions.0 * self.dimensions.1 {
            print!("{}, ", self.data[i]);
            if i % self.dimensions.0 == self.dimensions.0 - 1 {
                println!("");
            }
        }
    }

    pub fn print_vec3(&self) {
        for i in 0 .. self.dimensions.0 * self.dimensions.1 {
            let idx = i * 4;
            print!("({:>10.4}, {:>10.4}, {:>10.4}), ", self.data[idx], self.data[idx + 1], self.data[idx + 2]);
            if i % self.dimensions.0 == self.dimensions.0 - 1 {
                println!("");
            }
        }
    }
}

fn main() {
    let dimensions = (256, 256);
    let mut context = Context::new(dimensions);

    let dimensions = (5, 5);
    let mut initializer = Initializer::new(&context);
    let mut presenter = Presenter::new(&context);
    let mut simulator = Simulator::new(&context, dimensions);
    let interactor = Interactor::new(&context);

    let diffusion = 1.0;
    let viscosity = 0.0000001;
    let mut fluid = Fluid::new(&context, dimensions, diffusion, viscosity);

    initializer.initialize(&context, &mut fluid);
    let field = Field::new(fluid.density_field.data(), dimensions);
    field.print_float();

    let delta_time = 0.2;
    while context.context.run() {
        interactor.interact(&mut fluid);
        simulator.simulate(&mut fluid, delta_time);
        presenter.present(&context, &fluid);
    }
    while context.context.run() {}
}
