#version 450

// Ax = b
layout(r32f, location = 0) uniform image2D outputField;
layout(r32f, location = 1) uniform image2D xField;
layout(r32f, location = 2) uniform image2D bField;
layout(location = 3) uniform float alpha;
layout(location = 4) uniform float reciprocalBeta;
layout(location = 5) uniform ivec2 offset;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;

    vec4 xLeft   = imageLoad(xField, coordinate - ivec2(1, 0));
    vec4 xRight  = imageLoad(xField, coordinate + ivec2(1, 0));
    vec4 xBottom = imageLoad(xField, coordinate - ivec2(0, 1));
    vec4 xTop    = imageLoad(xField, coordinate + ivec2(0, 1));

    vec4 bCenter = imageLoad(bField, coordinate);

    vec4 value = (xLeft + xRight + xBottom + xTop + alpha * bCenter) * reciprocalBeta;

    imageStore(outputField, coordinate, value);
}
