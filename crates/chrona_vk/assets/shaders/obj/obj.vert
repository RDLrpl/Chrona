#version 450

layout(location = 0) in vec3 vecposition;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 color;

layout(set = 0, binding = 0) uniform CameraUBO {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(push_constant) uniform PushConstants {
    mat4 model;
} tr;

layout(location = 0) out vec3 frag_color;

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * tr.model * vec4(vecposition, 1.0);
    frag_color = color;
}