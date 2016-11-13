
uniform vec3 Ka;
uniform float d;
uniform sampler2D u_map_Ka;

in vec3 t_pos;
in vec2 t_uv;
in vec3 t_normal;

out vec4 out_col;

void main() {
	out_col = vec4(Ka, d) * texture2D(u_map_Ka, t_uv);
}