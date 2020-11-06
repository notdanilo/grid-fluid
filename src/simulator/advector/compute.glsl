#version 450

layout(location = 0) uniform ivec2 offset;
layout(rgba32f, location = 1) uniform image2D field;
layout(rgba32f, location = 2) uniform image2D previousField;
layout(rgba32f, location = 3) uniform image2D velocityField;
layout(location = 4) uniform float deltaTime;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;
    ivec2 size = imageSize(field);
    vec2 deltaTimeSize = deltaTime * (size - ivec2(2, 2));

    // FIXME: Considering size.x == size.y == ...
    float N = float(size.x);

    vec4 velocity = imageLoad(velocityField, coordinate);
    vec2 tmp = velocity.xy * deltaTimeSize;
    vec2 xy = clamp(vec2(coordinate) - tmp, vec2(0.5), vec2(N + 0.5));

    vec2 ij0 = floor(xy);
    vec2 ij1 = ij0 + vec2(1.0, 1.0);

    vec2 st1 = xy - ij0;
    vec2 st0 = vec2(1.0, 1.0) + st1;

    ivec2 coordinate0 = ivec2(ij0);
    ivec2 coordinate1 = ivec2(ij1);

    vec4 value =
        st0.x * (st0.y * imageLoad(previousField, ivec2(ij0.x, ij0.y)) + st1.y * imageLoad(previousField, ivec2(ij0.x, ij1.y))) +
        st1.x * (st0.y * imageLoad(previousField, ivec2(ij1.x, ij0.y)) + st1.y * imageLoad(previousField, ivec2(ij1.x, ij1.y)));

// s0 * (t0 * d0[IX(i0i, j0i)] + t1 * d0[IX(i0i, j1i)]) +
// s1 * (t0 * d0[IX(i1i, j0i)] + t1 * d0[IX(i1i, j1i)]);


    imageStore(field, coordinate, value);
}

// static void advect(int b, float *d, float *d0,  float *velocX, float *velocY, float *velocZ, float dt, int N) {
//    float i0, i1, j0, j1, k0, k1;
//
//    float dtx = dt * (N - 2);
//    float dty = dt * (N - 2);
//    float dtz = dt * (N - 2);
//
//    float s0, s1, t0, t1, u0, u1;
//    float tmp1, tmp2, tmp3, x, y, z;
//
//    float Nfloat = N;
//    float ifloat, jfloat, kfloat;
//    int i, j, k;
//
//    for(k = 1, kfloat = 1; k < N - 1; k++, kfloat++) {
//        for(j = 1, jfloat = 1; j < N - 1; j++, jfloat++) {
//            for(i = 1, ifloat = 1; i < N - 1; i++, ifloat++) {
//                tmp1 = dtx * velocX[IX(i, j, k)];
//                tmp2 = dty * velocY[IX(i, j, k)];
//                tmp3 = dtz * velocZ[IX(i, j, k)];
//                x    = ifloat - tmp1;
//                y    = jfloat - tmp2;
//                z    = kfloat - tmp3;
//
//                if(x < 0.5f) x = 0.5f;
//                if(x > Nfloat + 0.5f) x = Nfloat + 0.5f;
//                i0 = floorf(x);
//                i1 = i0 + 1.0f;
//                if(y < 0.5f) y = 0.5f;
//                if(y > Nfloat + 0.5f) y = Nfloat + 0.5f;
//                j0 = floorf(y);
//                j1 = j0 + 1.0f;
//                if(z < 0.5f) z = 0.5f;
//                if(z > Nfloat + 0.5f) z = Nfloat + 0.5f;
//                k0 = floorf(z);
//                k1 = k0 + 1.0f;
//
//                s1 = x - i0;
//                s0 = 1.0f - s1;
//                t1 = y - j0;
//                t0 = 1.0f - t1;
//                u1 = z - k0;
//                u0 = 1.0f - u1;
//
//                int i0i = i0;
//                int i1i = i1;
//                int j0i = j0;
//                int j1i = j1;
//                int k0i = k0;
//                int k1i = k1;
//
//                d[IX(i, j, k)] =
//
//                s0 * ( t0 * (u0 * d0[IX(i0i, j0i, k0i)]
//                +u1 * d0[IX(i0i, j0i, k1i)])
//                +( t1 * (u0 * d0[IX(i0i, j1i, k0i)]
//                +u1 * d0[IX(i0i, j1i, k1i)])))
//                +s1 * ( t0 * (u0 * d0[IX(i1i, j0i, k0i)]
//                +u1 * d0[IX(i1i, j0i, k1i)])
//                +( t1 * (u0 * d0[IX(i1i, j1i, k0i)]
//                +u1 * d0[IX(i1i, j1i, k1i)])));
//            }
//        }
//    }
// }