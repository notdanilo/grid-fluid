#version 450

layout(rgba32f, location = 0) uniform image2D velocityField;
layout(r32f, location = 1) uniform image2D pField;
layout(r32f, location = 2) uniform image2D divField;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy);
    vec4 velocity = imageLoad(velocityField, coordinate);
    float p = imageLoad(pField, coordinate).x;
    float div = imageLoad(divField, coordinate).x;
    velocity = vec4(p, div, velocity.zw);
    imageStore(velocityField, coordinate, velocity);
}