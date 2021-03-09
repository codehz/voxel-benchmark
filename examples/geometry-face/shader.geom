#version 450

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

layout(location = 0) in uint gtex[];
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
  uint start = gtex[0] & 0xFFFF;
  uint texid = gtex[0] >> 16;
  // if (start != 0 && start != 1) return;
  for (uint i = 0; i < 4; i++) {
    gl_Position =
        transform(gl_in[0].gl_Position + vec4(faces[i + start * 4], 1.0));
    muv = vec3(fuv[i], float(texid));
    EmitVertex();
  }
  EndPrimitive();
}