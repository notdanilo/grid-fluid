#version 450

// Ax = b
layout(rgba32f, location = 0) uniform image2D x;
layout(rgba32f, location = 1) uniform image2D b;
layout(location = 2) uniform ivec2 offset;
layout(location = 3) uniform float alpha;
layout(location = 4) uniform float reciprocalBeta;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;

    vec4 xLeft   = imageLoad(x, coordinate - ivec2(1, 0));
    vec4 xRight  = imageLoad(x, coordinate + ivec2(1, 0));
    vec4 xBottom = imageLoad(x, coordinate - ivec2(0, 1));
    vec4 xTop    = imageLoad(x, coordinate + ivec2(0, 1));

    vec4 bCenter = imageLoad(b, coordinate);

    vec4 value = (xLeft + xRight + xBottom + xTop + alpha * bCenter) * reciprocalBeta;

    imageStore(x, coordinate, value);
}

// float cRecip = 1.0 / c;
// for (int k = 0; k < iter; k++) {
//     for (int m = 1; m < N - 1; m++) {
//         for (int j = 1; j < N - 1; j++) {
//             for (int i = 1; i < N - 1; i++) {
//                 x[IX(i, j, m)] =
//                     (x0[IX(i, j, m)]
//                         + a*(    x[IX(i+1, j  , m  )]
//                                 +x[IX(i-1, j  , m  )]
//                                 +x[IX(i  , j+1, m  )]
//                                 +x[IX(i  , j-1, m  )]
//                                 +x[IX(i  , j  , m+1)]
//                                 +x[IX(i  , j  , m-1)]
//                        )) * cRecip;
//             }
//         }
//     }
//     set_bnd(b, x, N);
// }