use gpu::*;

mod initialize;

fn main() {
    let display = ContextDisplay::Window("Fluid".to_string(), 512, 512);
    let mut context = ContextBuilder::new().with_display(display).build();

    context.make_current().ok();

    let vertex_shader = VertexShader::new(&context, include_str!("vertex.glsl")).expect("Couldn't create VertexShader.");
    let fragment_shader = FragmentShader::new(&context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
    let mut raster_program = RasterProgram::new(&context, &fragment_shader, &vertex_shader).expect("Couldn't create RasterProgram.");

    let vertex_array_object = VertexArrayObject::new(&context);
    let framebuffer = Framebuffer::default(&context);

    let color_format = ColorFormat::RGBA;
    let component_type = Type::F32;
    let texture_format = TextureFormat::new(color_format, component_type);

    let mut data = Vec::new();
    for _ in 0 .. 512*512 {
        data.push(1.0);
        data.push(1.0);
        data.push(1.0);
        data.push(1.0);
    }
    let texture_2d = Texture2D::from_data(&context, (512,512), &texture_format, &data, &texture_format);
    let sampler = Sampler::new(&context);

    while context.run() {
        raster_program.bind_texture_2d(&texture_2d, &sampler, 0);
        raster_program.raster(&framebuffer, &vertex_array_object, RasterGeometry::Points, 1);
        context.swap_buffers().ok();
    }
    println!("Hello, world!");
}
