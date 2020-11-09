#version 460

layout(rg32f, location = 0) writeonly uniform image2D field;
layout(rg32f, location = 1) uniform image2D previousField;
layout(rg32f, location = 2) uniform image2D velocityField;
layout(location = 3) uniform float deltaTime;
layout(location = 4) uniform ivec2 offset;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

vec4 bilinearLoad(vec2 coordinate) {
    vec2 interpolation = fract(coordinate);
    ivec2 leftBottom   = ivec2(floor(coordinate));
    ivec2 rightTop     = ivec2(ceil(coordinate));
    ivec2 rightBottom  = ivec2(rightTop.x, leftBottom.y);
    ivec2 leftTop      = ivec2(leftBottom.x, rightTop.y);

    vec4 leftBottomValue  = imageLoad(previousField, leftBottom);
    vec4 rightBottomValue = imageLoad(previousField, rightBottom);
    vec4 leftTopValue     = imageLoad(previousField, leftTop);
    vec4 rightTopValue    = imageLoad(previousField, rightTop);
    vec4 bottomValue      = mix(leftBottomValue, rightBottomValue, interpolation.x);
    vec4 topValue         = mix(leftTopValue, rightTopValue, interpolation.y);
    return mix(bottomValue, topValue, interpolation.y);
}

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy) + offset;
    vec2 velocity = imageLoad(velocityField, coordinate).xy;
    vec2 previousCoordinate = vec2(coordinate) - velocity * deltaTime;
    vec4 value = bilinearLoad(previousCoordinate);
    imageStore(field, coordinate, value);
}