#version 460

layout(rgba32f, location = 0) uniform image2D velocity;
layout(r32f, location = 1) uniform image2D density;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    imageStore(velocity, coord, vec4(0.0));
    imageStore(density, coord, vec4(0.0));
}