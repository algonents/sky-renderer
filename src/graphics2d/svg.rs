use crate::graphics2d::shapes::ShapeRenderable;

pub struct SvgDocument {
    width: f32,
    height: f32,
    elements: Vec<String>,
}

impl SvgDocument {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: &ShapeRenderable) {
        self.elements.push(shape.to_svg());
    }

    pub fn add_shapes(&mut self, shapes: &[ShapeRenderable]) {
        for shape in shapes {
            self.add_shape(shape);
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     width="{w}" height="{h}">"#,
            w = self.width,
            h = self.height
        );

        for el in &self.elements {
            output.push_str(el);
        }

        output.push_str("</svg>");
        output
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_string())
    }
}
