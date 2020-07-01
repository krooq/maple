#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in float a_mix_factor;

layout(location=0) out vec4 v_color;
layout(location=1) out vec2 v_tex_coords;
layout(location=2) out float v_mix_factor;

layout(set=1, binding=0) uniform Uniforms { mat4 u_view_proj; };

layout(set=2, binding=0) buffer Instances { mat4 s_models[]; };

void main() {
    v_tex_coords = a_tex_coords;
    v_color = a_color;
    v_mix_factor = a_mix_factor;
    gl_Position = u_view_proj * s_models[gl_InstanceIndex] * vec4(a_position, 1.0);
}