#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in uvec3 comp_info;

layout(location = 0) out uvec3 gcomp;

void main() {
  gcomp = comp_info;
  gl_Position = vec4(position, 0.0);
}