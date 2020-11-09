#version 450

layout(rg32f, location = 0) uniform image2D field;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

#define DIMENSIONS 2
const ivec2 neighbors_offsets[DIMENSIONS] = {
    ivec2(1, 0),
    ivec2(0, 1)
};

void main() {
    ivec2 unitCoordinate = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = ivec2(imageSize(field));
    ivec2 coordinate = unitCoordinate * (size - ivec2(1, 1));

    vec4 value = vec4(0.0);
    for (int i = 0; i < DIMENSIONS; i++) {
        ivec2 offset = neighbors_offsets[i];
        ivec2 neighborCoordinate = coordinate + (offset - 2 * unitCoordinate) * offset;
        value += imageLoad(field, neighborCoordinate);
    }
    value /= float(DIMENSIONS);
    imageStore(field, coordinate, value);
}