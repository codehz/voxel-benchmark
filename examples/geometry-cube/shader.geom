#version 450

layout(points) in;
layout(triangle_strip, max_vertices = 24) out;

layout(location = 0) in uvec3 gcomp[];
layout(location = 0) out vec3 muv;

layout(location = 0) uniform mat4 perspective;
layout(location = 1) uniform mat4 view_model;

// clang-format off
vec3 faces[24] = vec3[24](
  // North
  vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0),
  // South
  vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 1.0), vec3(0.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0),
  // East
  vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, 0.0),
  // West
  vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 1.0),
  // Up
  vec3(0.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 0.0),
  // Down
  vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 1.0)
);
// clang-format on

vec2 fuv[4] =
    vec2[4](vec2(0.0, 1.0), vec2(1.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 0.0));

vec4 transform(vec4 point) { return perspective * view_model * point; }

void main() {
  uint tex[6];
  tex[0] = gcomp[0].r >> 16;
  tex[1] = gcomp[0].r & 0xFFFF;
  tex[2] = gcomp[0].g >> 16;
  tex[3] = gcomp[0].g & 0xFFFF;
  tex[4] = gcomp[0].b >> 16;
  tex[5] = gcomp[0].b & 0xFFFF;
  for (uint face = 0; face < 6; face++) {
    for (uint idx = 0; idx < 4; idx++) {
      gl_Position =
        transform(gl_in[0].gl_Position + vec4(faces[face * 4 + idx], 1.0));
      muv = vec3(fuv[idx], float(tex[idx]));
      EmitVertex();
    }
    EndPrimitive();
  }
}