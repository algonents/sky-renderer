extern crate sky_renderer;

use sky_renderer::core::{App, Window};

fn main() {
    let mut  window = Window::new("Hello Window", 800, 600);
    window.set_on_resize(move |w, h| {
        println!("window resized, width:{}, height: {}", w, h);
    });

    let app = App::new(window);
    app.run();
}
