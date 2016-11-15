
uniform mat4 u_mvp;
uniform mat4 u_model_mat;

in vec3 pos;
in vec2 uv;
in vec3 normal;

out vec3 t_pos;
out vec2 t_uv;
out vec3 t_normal;

out vec4 t_light_pos;

void main() {
	t_pos = vec3(u_model_mat * vec4(pos, 1.0));
	t_uv = uv;
	t_normal = vec3(u_model_mat * vec4(normal, 0.0));
	gl_Position = u_mvp * vec4(pos, 1.0);
}