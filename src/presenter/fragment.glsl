#version 450

layout(rgba32f, location = 0) uniform image2D velocity;
layout(r32f, location = 1) uniform image2D density;

out vec4 color;

void main() {
    ivec2 coord = ivec2(gl_FragCoord.xy);
    float density = imageLoad(density, coord).x;
    vec3 velocity = imageLoad(velocity, coord).xyz;
    color = vec4(vec3(density), 1.0);
}