use crate::core::Window;

pub struct App<'a> {
    pub window: Box<Window>,
    render_callback: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> App<'a> {
    pub fn new(window: Box<Window>) -> Self {
        Self {
            window,
            render_callback: None,
        }
    }
    
    pub fn on_render<F>(&mut self, callback: F)
    where
        F: FnMut() + 'a,
    {
        self.render_callback = Some(Box::new(callback));
    }

    pub fn run(mut self) {
        while !self.window.window_should_close() {
            self.window.clear_color(0.07, 0.13, 0.17, 1.0);

            if let Some(cb) = self.render_callback.as_mut() {
                cb();
            }

            self.window.swap_buffers();
            self.window.poll_events();
        }
    }
}