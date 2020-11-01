#version 450

layout(rgba32f, location = 0) uniform image2D velocityField;
layout(r32f, location = 1) uniform image2D pField;
layout(location = 2) uniform ivec2 offset;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

#define NUMBER_OF_COMPONENTS 2
const ivec2 offsets[NUMBER_OF_COMPONENTS] = {
    ivec2(1, 0),
    ivec2(0, 1)
};

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;
    vec4 velocity = imageLoad(velocityField, coordinate);

    for (int i = 0; i < NUMBER_OF_COMPONENTS; i++) {
        ivec2 offset = offsets[i];
        velocity[i] -= imageLoad(pField, coordinate + offset)[i] - imageLoad(pField, coordinate - offset)[i];
    }

    // Considering imageSize.x == imageSize.y, what happens if imageSize is rectangular?
    float N = float(imageSize(velocityField).x) * 0.5;
    velocity *= N;

    imageStore(velocityField, coordinate, velocity);
}

// Reference:
// for (int k = 1; k < N - 1; k++) {
//     for (int j = 1; j < N - 1; j++) {
//         for (int i = 1; i < N - 1; i++) {
//             velocX[IX(i, j, k)] -= 0.5f * (  p[IX(i+1, j, k)]
//                                             -p[IX(i-1, j, k)]) * N;
//             velocY[IX(i, j, k)] -= 0.5f * (  p[IX(i, j+1, k)]
//                                             -p[IX(i, j-1, k)]) * N;
//             velocZ[IX(i, j, k)] -= 0.5f * (  p[IX(i, j, k+1)]
//                                             -p[IX(i, j, k-1)]) * N;
//         }
//     }
// }