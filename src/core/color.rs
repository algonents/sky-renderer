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

    pub fn red(&self)->f32{
        self.r
    }

    pub fn green(&self)->f32{
        self.g
    }

    pub fn blue(&self)->f32{
        self.b
    }
    
    pub fn alpha(&self)->f32 { self.a }

    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.red() * 255.0) as u8,
            (self.green() * 255.0) as u8,
            (self.blue() * 255.0) as u8,
        )
    }
    
    
}