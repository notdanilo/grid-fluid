#version 450

layout(rgba32f, location = 0) uniform image2D field;
layout(location = 1) uniform int offset;
layout(location = 2) uniform ivec2 sideNormal;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
//    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;
//
//    ivec2 size = ivec2(imageSize(field)) - ivec2(1, 1);
//    ivec2 lowerCoordinate = coordinate * ivec2(coordinate.x, 0);
//    ivec2 upperCoordinate = ivec2(coordinate.x, size.y);
//    ivec2 leftCoordinate  = ivec2(0, coordinate.y);
//    ivec2 rightCoordinate = ivec2(size.x, coordinate.y);
//
//    vec4 lowerValue = imageLoad(field, lowerCoordinate + ivec2(0, 1));
//    vec4 upperValue = imageLoad(field, upperCoordinate - ivec2(0, 1));
//    imageStore(field, lowerCoordinate, lowerValue);
//    imageStore(field, upperCoordinate, upperValue);
//
//    vec4 leftValue  = imageLoad(field, leftCoordinate  + ivec2(1, 0));
//    vec4 rightValue = imageLoad(field, rightCoordinate - ivec2(1, 0));
//    imageStore(field, leftCoordinate, leftValue);
//    imageStore(field, rightCoordinate, rightValue);

    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = ivec2(imageSize(field)) - ivec2(1, 1);

    // If sideNormal = (1, 0), it means it`s the normal of the X sides.
    // Then the sideMask will be (0, 1), which means we will iterate over all the Y points.
    ivec2 sideMask = ivec2(1, 1) - sideNormal;
    for (int i = 0; i < 2; i++) {
        // In the X sides case, we will iterate from (0, offset.y) to (0, invocations.y) when i == 0
        // and from (size.y - 1, offset.y) to (size.y - 1, invocations.y) when i == 1.
        ivec2 sideCoordinate = (coordinate + offset) * sideMask + size * i;
        // The neighbor offset is (1, 0) if i == 0 and (-1, 0) if i == 1.
        ivec2 neighborOffset = sideNormal * (1 - i) - sideNormal * i;
        ivec2 neighborCoordinate = sideCoordinate + neighborOffset;
        vec4 value = imageLoad(field, neighborCoordinate);
        ivec2 componentNegator = ivec2(1, 1) - (2 * sideNormal);
        // We then negate the X component by multiplying with a (-1, 1) vector.
        value *= vec4(componentNegator, 1, 1);
        imageStore(field, sideCoordinate, value);
    }

    /* Reference
    // X sides
    for(int k = 1; k < N - 1; k++) {
        for(int j = 1; j < N - 1; j++) {
            x[IX(0  , j, k)] = b == 1 ? -x[IX(1  , j, k)] : x[IX(1  , j, k)];
            x[IX(N-1, j, k)] = b == 1 ? -x[IX(N-2, j, k)] : x[IX(N-2, j, k)];
        }
    }

    for(int k = 1; k < N - 1; k++) {
        for(int i = 1; i < N - 1; i++) {
            x[IX(i, 0  , k)] = b == 2 ? -x[IX(i, 1  , k)] : x[IX(i, 1  , k)];
            x[IX(i, N-1, k)] = b == 2 ? -x[IX(i, N-2, k)] : x[IX(i, N-2, k)];
        }
    }
    */
}