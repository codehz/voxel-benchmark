#version 450

layout(location = 0) in vec3 muv;
layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2DArray tile;

void main() {
  color = texture(tile, muv);
}