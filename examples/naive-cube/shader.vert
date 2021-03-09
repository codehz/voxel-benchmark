#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in uint tex_info;
layout(location = 0) out vec3 muv;

layout(location = 0) uniform mat4 perspective;
layout(location = 1) uniform mat4 view_model;

void main() {
  muv = vec3(float((tex_info & 2) >> 1), float(tex_info & 1), float(tex_info >> 16));
  gl_Position = perspective * view_model * vec4(position, 1.0);
}