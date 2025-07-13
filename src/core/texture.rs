use std::ffi::c_void;
use crate::core::image::{load_image, Image};
use crate::engine::opengl::{gl_bind_texture, gl_gen_texture, gl_generate_mipmap, gl_tex_image_2d, gl_tex_parameteri, GL_LINEAR, GL_LINEAR_MIPMAP_LINEAR, GL_REPEAT, GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE};

pub fn generate_texture_from_image(image: &Image) -> u32 {
    let texture = gl_gen_texture();
    gl_bind_texture(GL_TEXTURE_2D, texture);

    gl_tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    gl_tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    // set texture filtering parameters
    gl_tex_parameteri(
        GL_TEXTURE_2D,
        GL_TEXTURE_MIN_FILTER,
        GL_LINEAR_MIPMAP_LINEAR,
    );
    gl_tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    
    gl_tex_image_2d(
        GL_TEXTURE_2D,
        0,
        GL_RGBA as i32,
        image.width as i32,
        image.height as i32,
        0,
        GL_RGBA as u32,
        GL_UNSIGNED_BYTE,
        image.pixels.as_ptr() as *const c_void,
    );
    gl_generate_mipmap(GL_TEXTURE_2D);

    texture
}