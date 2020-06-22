#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;

layout(location=0) out vec3 v_color;


layout(set=1, binding=0)
uniform Uniforms {
    mat4 u_view_proj;
};

layout(set=1, binding=1)
buffer Instances {
    mat4 s_models[];
};

void main() {
    v_color = a_color;
    gl_Position = u_view_proj * s_models[gl_InstanceIndex] * vec4(a_position, 1.0);
}