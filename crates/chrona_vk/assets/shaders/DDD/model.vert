#version 450

layout(location = 0) in vec3 vecposition;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 color;

layout(location = 0) out vec2 f_uv;
layout(location = 1) out vec3 f_color;

layout(set = 0, binding = 0) uniform CameraUBO {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(push_constant) uniform PushConstants {
    mat4 model;
} tr;

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * tr.model * vec4(vecposition, 1.0);
    f_uv = uv;
    f_color = color; 
}