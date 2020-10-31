#version 450

layout(rgba32f, location = 0) uniform image2D currentField;
layout(rgba32f, location = 1) uniform image2D previousField;
layout(location = 2) uniform ivec2 offset;
layout(location = 3) uniform float a;
layout(location = 4) uniform float cReciprocal;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

#define NUMBER_OF_NEIGHBORS 4
const ivec2 neighborsOffsets[NUMBER_OF_NEIGHBORS] = {
    ivec2(-1,  0),
    ivec2( 1,  0),
    ivec2( 0,  1),
    ivec2( 0, -1)
};

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;

    vec4 value = vec4(0.0);
    for (int i = 0; i < NUMBER_OF_NEIGHBORS; i++) {
        value += imageLoad(currentField, coordinate + neighborsOffsets[i]);
    }
    value *= a;
    value += imageLoad(previousField, coordinate);
    value *= cReciprocal;

    imageStore(currentField, coordinate, value);
}