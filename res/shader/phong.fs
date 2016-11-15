
uniform vec3 u_Ka;
uniform float u_d;
uniform sampler2D u_map_Ka;

in vec3 t_pos;
in vec2 t_uv;
in vec3 t_normal;

out vec4 out_col;

void main() {
	out_col = vec4(u_Ka, u_d) * texture2D(u_map_Ka, t_uv);
}
