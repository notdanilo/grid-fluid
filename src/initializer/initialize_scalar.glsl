#version 460

layout(r32f, location = 0) uniform image2D field;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

const float PI = acos(-1.0);

float square(float x) { return sign(sin(x * PI)); }

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    vec2 position = (vec2(coord));
    float value = clamp(square(position.x / 32.0) * square(position.y / 32.0), 0.0, 1.0);
    imageStore(field, coord, vec4(value));
}