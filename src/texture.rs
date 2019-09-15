use vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

pub struct CheckerTexture {
    odd: Box<dyn Texture + Sync>,
    even: Box<dyn Texture + Sync>
}


impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture {
            color
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture + Sync>, even: Box<dyn Texture + Sync>) -> CheckerTexture {
        CheckerTexture {
            odd,
            even
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u,v,p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
