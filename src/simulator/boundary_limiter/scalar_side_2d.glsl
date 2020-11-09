#version 450

layout(r32f, location = 0) uniform image2D field;
layout(location = 1) uniform int offset;
layout(location = 2) uniform ivec2 sideNormal;
layout(location = 3) uniform bool isVelocityField;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = ivec2(imageSize(field)) - ivec2(1, 1);

    // If sideNormal = (1, 0), it means it`s the normal of the X sides.
    // Then the sideMask will be (0, 1), which means we will iterate over all the Y points.
    ivec2 sideMask = ivec2(1, 1) - sideNormal;
    for (int i = 0; i < 2; i++) {
        // In the X sides case, we will iterate from (0, offset.y) to (0, invocations.y) when i == 0
        // and from (size.y - 1, offset.y) to (size.y - 1, invocations.y) when i == 1.
        ivec2 sideCoordinate = (coordinate + offset) * sideMask + size * sideNormal * i;
        // The neighbor offset is (1, 0) if i == 0 and (-1, 0) if i == 1.
        ivec2 neighborOffset = sideNormal * (1 - i) - sideNormal * i;
        ivec2 neighborCoordinate = sideCoordinate + neighborOffset;
        vec4 value = imageLoad(field, neighborCoordinate);
        imageStore(field, sideCoordinate, value);
    }
}