use crate::core::engine::opengl::{
    GLuint, gl_attach_shader, gl_compile_shader, gl_create_fragment_shader,
    gl_create_geometry_shader, gl_create_program, gl_create_vertex_shader, gl_delete_program,
    gl_delete_shader, gl_link_program, gl_shader_source, gl_use_program,
};

pub struct Shader {
    program: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.program != 0 {
            gl_delete_program(self.program);
        }
    }
}

impl Shader {
    pub fn compile(
        vertex_src: &str,
        fragment_src: &str,
        geometry_src: Option<&str>,
    ) -> Result<Self, String> {
        let program = gl_create_program();

        let vertex_shader = gl_create_vertex_shader();
        gl_shader_source(vertex_shader, vertex_src);
        gl_compile_shader(vertex_shader);
        gl_attach_shader(program, vertex_shader);

        /*
        if !gl_get_shader_compile_status(vertex_shader) {
            return Err(gl_get_shader_info_log(vertex_shader));
        }*/

        let fragment_shader = gl_create_fragment_shader();
        gl_shader_source(fragment_shader, fragment_src);
        gl_compile_shader(fragment_shader);
        gl_attach_shader(program, fragment_shader);

        /*
        if !gl_get_shader_compile_status(fragment_shader) {
            return Err(gl_get_shader_info_log(fragment_shader));
        }*/

        let geometry_shader = if let Some(geometry_code) = geometry_src {
            let shader = gl_create_geometry_shader();
            gl_shader_source(shader, geometry_code);
            gl_compile_shader(shader);
            gl_attach_shader(program, shader);
            Some(shader)
        } else {
            None
        };

        gl_link_program(program);

        /*
        if !gl_get_program_link_status(program) {
            return Err(gl_get_program_info_log(program));
        }*/

        // Delete shader objects after linking - they're no longer needed
        gl_delete_shader(vertex_shader);
        gl_delete_shader(fragment_shader);
        if let Some(shader) = geometry_shader {
            gl_delete_shader(shader);
        }

        Ok(Self { program })
    }

    pub fn use_program(&self) {
        gl_use_program(self.program)
    }

    pub fn program(&self) -> GLuint {
        self.program
    }
}
