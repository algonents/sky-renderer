use std::ffi::{c_float, CString};
use std::ffi::c_void;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_int;

pub const GLFW_SAMPLES: i32 = 0x0002100D;
pub enum GLFWwindow {}

pub type GLFWframebuffersizefun =
    Option<extern "C" fn(window: *const GLFWwindow, width: i32, height: i32)>;

pub type GLFWwindowsizefun =
Option<extern "C" fn(window: *const GLFWwindow, width: i32, height: i32)>;

pub type GLFWscrollfun =
    Option<extern "C" fn(window: *const GLFWwindow, xoffset: f64, yoffset: f64)>;

pub type GLFWcursorposfun = Option<extern "C" fn(window: *const GLFWwindow, xpos: f64, ypos: f64)>;

unsafe extern "C" {
    fn _glfwCreateWindow(
        title: *const c_char,
        width: c_int,
        height: c_int,
        callback: GLFWframebuffersizefun,
    ) -> *const GLFWwindow;

    fn _glfwGetWindowContentScale(window: *const GLFWwindow, xscale: *mut c_float, yscale: *mut c_float);

    fn _glfwWindowHint(hint: c_int, value:c_int);

    fn _glfwSetWindowUserPointer(window: *const GLFWwindow, pointer: *const c_void);
    fn _glfwGetWindowUserPointer(window: *const GLFWwindow) -> *const c_void;

    fn _glfwWindowShouldClose(window: *const GLFWwindow) -> c_int;
    fn _glfwSwapBuffers(window: *const GLFWwindow);

    fn _glfwPollEvents();
    fn _glfwTerminate();

    fn _glfwGetTime() -> c_double;
    fn _glfwSetFramebufferSizeCallback(window: *const GLFWwindow, callback: GLFWframebuffersizefun);
    fn _glfwSetWindowSizeCallback(window: *const GLFWwindow, callback: GLFWwindowsizefun);
    fn _glfwSetScrollCallback(window: *const GLFWwindow, callback: GLFWscrollfun);
    fn _glfwSetCursorPosCallback(window: *const GLFWwindow, callback: GLFWcursorposfun);
    fn _glfwGetWindowSize(window: *const GLFWwindow, width: *mut c_int, height: *mut c_int);

}

pub fn glfw_get_time() -> f64 {
    unsafe { _glfwGetTime() }
}

pub fn glfw_create_window(
    title: &str,
    width: i32,
    height: i32,
    callback: GLFWframebuffersizefun,
) -> *const GLFWwindow {
    let window_pointer: *const GLFWwindow;
    let title_c_string = CString::new(title).expect("Failed to create title");
    unsafe {
        window_pointer = _glfwCreateWindow(title_c_string.as_ptr(), width, height, callback);
    }
    window_pointer
}

pub fn glfw_get_window_content_scale(window: *const GLFWwindow)->(f32, f32){
    unsafe {
        let mut xs: f32 = 0.0;
        let mut ys: f32 = 0.0;
        _glfwGetWindowContentScale(window, &mut xs, &mut ys);
        (xs, ys)
    }
}

pub fn glfw_window_hint(hint: i32, value: i32){
    unsafe{
        _glfwWindowHint(hint, value);
    }
}

pub fn glfw_set_window_user_pointer(window: *const GLFWwindow, pointer: *mut c_void) {
    unsafe {
        _glfwSetWindowUserPointer(window, pointer);
    }
}

pub fn glfw_get_window_user_pointer(window: *const GLFWwindow) -> *const c_void {
    unsafe { _glfwGetWindowUserPointer(window) }
}

pub fn glfw_window_should_close(window: *const GLFWwindow) -> bool {
    let result: i32;
    unsafe {
        result = _glfwWindowShouldClose(window);
    }
    result != 0
}

pub fn glfw_set_scroll_callback(window: *const GLFWwindow, callback: GLFWscrollfun) {
    unsafe {
        _glfwSetScrollCallback(window, callback);
    }
}

pub fn glfw_set_cursor_pos_callback(window: *const GLFWwindow, callback: GLFWcursorposfun) {
    unsafe {
        _glfwSetCursorPosCallback(window, callback);
    }
}

pub fn glfw_set_window_size_callback(window: *const GLFWwindow, callback: GLFWwindowsizefun){
    unsafe{
        _glfwSetWindowSizeCallback(window, callback);
    }
}

pub fn glfw_get_window_size(window: *const GLFWwindow, width: *mut c_int, height: *mut c_int) {
    unsafe {
        _glfwGetWindowSize(window, width, height);
    }
}

pub fn glfw_poll_events() {
    unsafe { _glfwPollEvents() }
}

pub fn glfw_swap_buffers(window: *const GLFWwindow) {
    unsafe { _glfwSwapBuffers(window) }
}

pub fn glfw_terminate() {
    unsafe { _glfwTerminate() };
}
