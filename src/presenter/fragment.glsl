#version 450

layout(rgba32f, location = 0) uniform image2D velocity;
layout(rgba32f, location = 1) uniform image2D density;
layout(location = 2) uniform vec2 resolution;
layout(location = 3) uniform vec2 fieldResolution;

out vec4 color;

void main() {
    ivec2 coord = ivec2(gl_FragCoord.xy / resolution * fieldResolution);
    vec3 density = imageLoad(density, coord).xyz;
    vec3 velocity = imageLoad(velocity, coord).xyz;
    color = vec4(density, 1.0);
 }