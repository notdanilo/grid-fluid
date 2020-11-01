use crate::context::Context;

pub struct Advector {

}

impl Advector {
    pub fn new(context: &Context) -> Self {
        Self {}
    }

    pub fn advect(&self, field: &mut gpu::Texture2D, previous_field: &gpu::Texture2D, velocity_field: &gpu::Texture2D) {

    }
}