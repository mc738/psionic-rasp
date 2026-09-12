use crate::core::InternalIdMap;

pub struct ResourcesMap {
    pub materials_map: InternalIdMap,
    pub textures_map: InternalIdMap,
    pub shaders_map: InternalIdMap,
    pub models_map: InternalIdMap,
    pub meshes_map: InternalIdMap,
    pub mesh_primitives_map: InternalIdMap,
    pub renderable_objects_map: InternalIdMap,
}

impl ResourcesMap {
    pub fn blank() -> Self {
        Self {
            materials_map: InternalIdMap::new(),
            textures_map: InternalIdMap::new(),
            shaders_map: InternalIdMap::new(),
            models_map: InternalIdMap::new(),
            meshes_map: InternalIdMap::new(),
            mesh_primitives_map: InternalIdMap::new(),
            renderable_objects_map: InternalIdMap::new(),
        }
    }

    pub fn build(
        shaders_map: &InternalIdMap,
        textures_map: &InternalIdMap,
        materials_map: &InternalIdMap,
        models_map: &InternalIdMap,
        meshes_map: &InternalIdMap,
        mesh_primitives_map: &InternalIdMap,
        renderable_objects_map: &InternalIdMap,
    ) -> ResourcesMap {
        Self {
            materials_map: materials_map.clone(),
            textures_map: textures_map.clone(),
            shaders_map: shaders_map.clone(),
            models_map: models_map.clone(),
            meshes_map: meshes_map.clone(),
            mesh_primitives_map: mesh_primitives_map.clone(),
            renderable_objects_map: renderable_objects_map.clone(),
        }
    }
}
