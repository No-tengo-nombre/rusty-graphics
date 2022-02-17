use std::fs;
use ogl33::{
    glCreateShader,
    glCompileShader,
    glAttachShader,
    glDeleteShader,
    glShaderSource,
    glGetShaderInfoLog,
    glGetProgramInfoLog,
    glGetShaderiv,
    glGetProgramiv,
    glCreateProgram,
    glLinkProgram,
    glUseProgram,
    GLuint,
    GL_VERTEX_SHADER,
    GL_FRAGMENT_SHADER,
    GL_COMPILE_STATUS,
    GL_LINK_STATUS,
};


pub struct Shader {
    _id: u32,
}


impl Shader {
    pub fn use_program(&self) {
        unsafe {
            glUseProgram(self._id);
        }
    }
}


pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
    // Making the vertex and fragment shaders
    let vertex_content = fs::read_to_string(vertex_path)
        .expect("Error reading vertex shader.");
    let fragment_content = fs::read_to_string(fragment_path)
        .expect("Error reading fragment shader.");
    let vertex_shader = make_vertex_shader(&vertex_content);
    verify_vertex_shader(&vertex_shader);
    let fragment_shader = make_fragment_shader(&fragment_content);
    verify_fragment_shader(&fragment_shader);

    // Creatig the shader program (by default it is not used)
    let shader_program;
    unsafe {
        shader_program = glCreateProgram();
        assert_ne!(shader_program, 0);
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        let mut success = 0;
        glGetProgramiv(
            shader_program,
            GL_LINK_STATUS,
            &mut success,
        );
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
    }
    return Shader {_id: shader_program};
}


fn make_vertex_shader(content: &String) -> GLuint{
    let vertex_shader;
    unsafe {
        vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        glShaderSource(
            vertex_shader,
            1,
            &(content.as_bytes().as_ptr().cast()),
            &(content.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
    }
    return vertex_shader;
}


fn make_fragment_shader(content: &String) -> GLuint{
    let fragment_shader;
    unsafe {
        fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(
            fragment_shader,
            1,
            &(content.as_bytes().as_ptr().cast()),
            &(content.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
    }
    return fragment_shader;
}


fn verify_vertex_shader(vertex_shader: &GLuint) {
    let mut success = 0;
    unsafe {
        glGetShaderiv(*vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                *vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
}


fn verify_fragment_shader(fragment_shader: &GLuint) {
    let mut success = 0;
    unsafe {
        glGetShaderiv(*fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                *fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
}
