#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 color;

layout(push_constant) uniform PushConstants {
    mat4 transform;
} pc;

layout(location = 0) out vec3 frag_color;

void main() {
    gl_Position = pc.transform * vec4(position, 1.0);
    frag_color = color;
}