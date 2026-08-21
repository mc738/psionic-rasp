pub enum  Material {
    Basic(BasicMaterial),
    Unlit(UnlitMaterial),
}

pub struct BasicMaterial {
    pub shader_internal_id: u32
}

pub struct UnlitMaterial {
    pub shader_internal_id: u32,
    pub texture_internal_id: u32,
}