
uniform mat4 u_mvp;

in vec3 in_pos;
in vec2 in_uv;
in vec3 in_normal;

out vec3 t_pos;
out vec2 t_uv;
out vec3 t_normal;

void main() {
	t_pos = in_pos;
	t_uv = in_uv;
	t_normal = in_normal;
	gl_Position = u_mvp * in_pos;
}