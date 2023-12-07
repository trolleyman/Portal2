#version 440

uniform vec3 u_color;
uniform float u_d;
uniform sampler2D u_map;

in vec3 t_pos;
in vec2 t_uv;
in vec3 t_normal;

out vec4 out_col;

void main() {
	out_col = vec4(u_color, u_d) * texture2D(u_map, t_uv);
}
