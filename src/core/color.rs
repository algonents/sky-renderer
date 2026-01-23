#[derive(Clone, Debug)]
pub struct Color{
    r:f32,
    g:f32,
    b:f32,
    a: f32
}

impl Color{
    pub fn from_rgb(r:f32, g:f32, b:f32)->Self{
        Color {r, g, b, a:1.0}
    }

    pub fn red_value(&self)->f32{
        self.r
    }

    pub fn green_value(&self)->f32{
        self.g
    }

    pub fn blue_value(&self)->f32{
        self.b
    }

    pub fn alpha(&self)->f32 { self.a }

    pub fn black() -> Self {
        Color::from_rgb(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::from_rgb(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Color::from_rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Color::from_rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Color::from_rgb(0.0, 0.0, 1.0)
    }


    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }
    
    
}