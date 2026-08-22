pub enum  Material {
    Basic(BasicMaterial),
    Unlit(UnlitMaterial),
}

impl Material {
    
    pub fn is_transparent(&self) -> bool {
        match self {
            Material::Basic(bm) => bm.is_transparent,
            Material::Unlit(um) => um.is_transparent
        }
    }
}

pub struct BasicMaterial {
    pub shader_internal_id: u32,
    pub is_transparent: bool,
}

pub struct UnlitMaterial {
    pub shader_internal_id: u32,
    pub texture_internal_id: u32,
    pub is_transparent: bool,
}