extern crate sky_renderer;

use sky_renderer::core::{App, Window};

fn main() {
    let window = Window::new("Hello Window", 800, 600);
    let app = App::new(window);

    app.run();
}
