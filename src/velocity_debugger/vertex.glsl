#version 460

layout(location = 0) in vec2 inVertex;
layout(location = 0) uniform vec2 position;
layout(rg32f, location = 1) uniform image2D velocityField;

mat2 rotate(float a) {
    return mat2( cos(a), -sin(a),
                 sin(a),  cos(a));
}

void main(void) {
    float scale = imageSize(velocityField).x / 50.0;
    vec2 velocity = imageLoad(velocityField, ivec2(position)).xy;
    float a = atan(velocity.y, velocity.x);
    vec2 vertex = inVertex * vec2(length(velocity) / 100.0, 1.0) * rotate(a) + position;
    gl_Position = vec4(vertex / scale, 0.0, 1.0);
}
