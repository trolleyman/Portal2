
uniform mat4 u_mvp;

in vec3 pos;
in vec2 uv;
in vec3 normal;

out vec3 t_pos;
out vec2 t_uv;
out vec3 t_normal;

void main() {
	t_pos = pos;
	t_uv = uv;
	t_normal = normal;
	gl_Position = u_mvp * vec4(pos, 1.0);
}