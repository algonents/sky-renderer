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
}