#version 450

layout(rgba32f, location = 0) uniform image2D velocityField;
layout(rgba32f, location = 1) uniform image2D densityField;
layout(location = 2) uniform ivec2 offset;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;
    imageStore(densityField, coordinate, vec4(1.0));
    imageStore(velocityField, coordinate, vec4(0.0, 0.0, 0.0, 0.0));
}