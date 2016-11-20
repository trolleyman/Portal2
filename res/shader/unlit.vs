
uniform mat4 u_mvp;
uniform mat4 u_model_mat;

uniform vec2 u_map_uv_scale;

in vec3 pos;
in vec2 uv;
in vec3 normal;

out vec3 t_pos;
out vec2 t_uv;
out vec3 t_normal;

void main() {
	// Transform position into world space
	t_pos = vec3(u_model_mat * vec4(pos, 1.0));
	
	// Calculate uvs
	t_uv = uv * u_map_uv_scale;
	
	// Transform normals into world space
	t_normal = vec3(u_model_mat * vec4(normal, 0.0));
	
	// Set actual position
	gl_Position = u_mvp * vec4(pos, 1.0);
}
