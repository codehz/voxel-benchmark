#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in uint tex_info;

layout(location = 0) out uint gtex;

void main() {
  gtex = tex_info;
  gl_Position = vec4(position, 0.0);
}