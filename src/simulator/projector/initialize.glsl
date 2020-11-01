#version 450

layout(rgba32f, location = 0) uniform image2D velocityField;
layout(r32f, location = 1) uniform image2D pField;
layout(r32f, location = 2) uniform image2D divField;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

#define NUMBER_OF_DIMENSIONS 2
#define NUMBER_OF_NEIGHBORS 4
const ivec2 neighborsOffsets[NUMBER_OF_NEIGHBORS] = {
    ivec2(-1,  0),
    ivec2( 1,  0),
    ivec2( 0,  1),
    ivec2( 0, -1)
};

const float neighborsFactors[NUMBER_OF_NEIGHBORS] = {
    -1.0,
     1.0,
     1.0,
    -1.0
};

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy);

    imageStore(pField, coordinate, vec4(0.0));
    vec4 div = vec4(0.0);
    for (int i = 0; i < NUMBER_OF_NEIGHBORS; i++) {
        int component = i / NUMBER_OF_DIMENSIONS;
        ivec2 neighborCoordinate = coordinate + neighborsOffsets[i];
        div += imageLoad(velocityField, neighborCoordinate)[component] * neighborsFactors[i];
    }
    // FIXME: It assumes the field size is squared, what happens if we use rectangular sizes?
    div = -0.5 * div / float(imageSize(velocityField).x);
    imageStore(divField, coordinate, div);
}

// Reference:
// div[IX(i, j, k)] = -0.5f*(
//          velocX[IX(i+1, j  , k  )]
//         -velocX[IX(i-1, j  , k  )]
//         +velocY[IX(i  , j+1, k  )]
//         -velocY[IX(i  , j-1, k  )]
//         +velocZ[IX(i  , j  , k+1)]
//         -velocZ[IX(i  , j  , k-1)]
//     )/N;
// p[IX(i, j, k)] = 0;
