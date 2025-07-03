use crate::engine::opengl::{gl_clear_color, gl_viewport};
use crate::windowing::glfw::{glfw_create_window, glfw_get_window_user_pointer, glfw_poll_events, glfw_set_window_user_pointer, glfw_swap_buffers, glfw_window_should_close, GLFWwindow};
use std::ffi::c_void;

pub struct Window{
    width: i32,
    height: i32,
    glfw_window: *const GLFWwindow,
    on_resize: Option<Box<dyn FnMut(i32, i32)>>,
}

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
    let user_ptr = glfw_get_window_user_pointer(_window);
    if !user_ptr.is_null() {
        unsafe {
            let window_ref: &mut Window = &mut *(user_ptr as *mut Window);
            window_ref._on_resize(width, height);
        }
    }
}


impl Window{
    pub fn new(title: &str, width: i32, height: i32)->Box<Self>{
        let glfw_window = glfw_create_window(title, width, height, Some(on_viewport_resized));
        let mut window = Box::new(Window{glfw_window, width, height,  on_resize: None});
        glfw_set_window_user_pointer(glfw_window, &mut *window as *mut _ as *mut c_void);
        window
    }
    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        gl_clear_color(red, green, blue, alpha);
    }
    pub fn window_should_close(&self) -> bool {
        glfw_window_should_close(self.glfw_window)
    }
    pub fn swap_buffers(&self) {
        glfw_swap_buffers(self.glfw_window);
    }
    pub fn poll_events(&self) {
        glfw_poll_events();
    }

    pub fn set_on_resize<F>(&mut self, f: F)
    where
        F: FnMut(i32, i32) + 'static,
    {
        self.on_resize = Some(Box::new(f));
    }
    pub fn _on_resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        if let Some(callback) = &mut self.on_resize {
            callback(width, height);
        }
    }
}