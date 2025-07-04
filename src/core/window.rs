use crate::engine::opengl::{gl_clear_color, gl_viewport};
use crate::engine::glfw::{glfw_create_window, glfw_get_window_user_pointer, glfw_poll_events, glfw_set_scroll_callback, glfw_set_window_user_pointer, glfw_swap_buffers, glfw_window_should_close, GLFWwindow};
use std::ffi::c_void;

pub struct Window{
    width: i32,
    height: i32,
    glfw_window: *const GLFWwindow,
    on_resize: Option<Box<dyn FnMut(i32, i32)>>,
    on_scroll: Option<Box<dyn FnMut(f64, f64)>>,
}

extern "C" fn _on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
    let user_ptr = glfw_get_window_user_pointer(_window);
    if !user_ptr.is_null() {
        unsafe {
            let window_ref: &mut Window = &mut *(user_ptr as *mut Window);
            window_ref._on_resize(width, height);
        }
    }
}

extern "C" fn _on_scroll_callback(_window: *const GLFWwindow, x_offset: f64, y_offset: f64) {
    let user_ptr = glfw_get_window_user_pointer(_window);
    if !user_ptr.is_null() {
        unsafe {
            let window_ref: &mut Window = &mut *(user_ptr as *mut Window);
            window_ref._on_scroll(x_offset, y_offset);
        }
    }
}


impl Window{
    pub fn new(title: &str, width: i32, height: i32)->Box<Self>{
        let glfw_window = glfw_create_window(title, width, height, Some(_on_viewport_resized));
        glfw_set_scroll_callback(glfw_window, Some(_on_scroll_callback));
        let mut window = Box::new(Window{glfw_window, width, height,  on_resize: None, on_scroll: None});
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

    pub fn on_resize<F>(&mut self, f: F)
    where
        F: FnMut(i32, i32) + 'static,
    {
        self.on_resize = Some(Box::new(f));
    }

    pub fn on_scroll<F>(&mut self, f: F)
    where
        F: FnMut(f64, f64) + 'static,
    {
        self.on_scroll = Some(Box::new(f));
    }
    
    
    fn _on_resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        if let Some(callback) = &mut self.on_resize {
            callback(width, height);
        }
    }

    fn _on_scroll(&mut self, x_offset: f64, y_offset: f64) {
        if let Some(callback) = &mut self.on_scroll {
            callback(x_offset, y_offset);
        }
    }
}