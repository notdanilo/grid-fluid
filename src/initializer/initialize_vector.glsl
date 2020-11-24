#version 460

layout(rg32f, location = 0) uniform image2D field;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    vec2 position = vec2(coord) / 10.0;
//    vec2 direction = vec2(sin(position.y), sin(position.x)) * 100.0;
    vec2 direction = vec2(1.0, sin(position.x * 2.0)) * 100.0;
    imageStore(field, coord, vec4(direction, 0.0, 0.0));
}
