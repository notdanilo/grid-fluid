#version 450

layout(location = 0) uniform sampler2D sampler;

void main() {
    vec2 uv = gl_FragCoord.xy / vec2(512.0, 512.0);
    gl_FragColor = texture(sampler, uv);
}