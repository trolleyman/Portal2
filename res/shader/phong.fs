
uniform vec3 u_Ka;
uniform vec3 u_Kd;
uniform float u_d;
uniform sampler2D u_map_Ka;
uniform sampler2D u_map_Kd;

uniform vec4 u_light_ambient;
uniform vec4 u_light_diffuse;
uniform vec3 u_light_pos;

in vec3 t_pos;
in vec2 t_uv;
in vec3 t_normal;

out vec4 out_col;

void main() {
	// l points from the surface to the light
	vec3 l = u_light_pos - t_pos;
	// To get the brightness, we calculate angle of incidence.
	float diffuse_brightness = clamp(dot(t_normal, l), 0.0, 1.0);
	
	vec4 ambient = vec4(u_Ka, u_d) * texture2D(u_map_Ka, t_uv) * u_light_ambient;
	vec4 diffuse = vec4(u_Kd, u_d) * texture2D(u_map_Kd, t_uv) * u_light_diffuse * diffuse_brightness;
	out_col = ambient + diffuse;
}
