use crate::core::InternalIdMap;
use crate::maths::Transform;
use crate::rendering::NewRendererResources;
use crate::rendering::materials::{BasicMaterial, Material, UnlitMaterial};
use crate::rendering::models::{
    Mesh, MeshInternalId, MeshPrimitive, MeshPrimitiveInternalId, Model, NewModelStoreResources,
};
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use crate::templates::{MaterialTemplate, MaterialTemplateType, MeshPrimitiveTemplate, SceneTemplate};
use glow::Context;
use std::collections::HashMap;
use std::mem;
use uuid::Uuid;
use crate::camera::Camera;

pub struct SceneInstance {
    pub graph: SceneGraph,
    pub transforms: TransformsCollection,
    pub main_camera: Camera,
}

pub struct ResourcesMap {
    pub materials_map: InternalIdMap,
    pub textures_map: InternalIdMap,
    pub shaders_map: InternalIdMap,
    pub models_map: InternalIdMap,
    pub meshes_map: InternalIdMap,
    pub mesh_primitives_map: InternalIdMap,
}

pub type TransformInternalId = u32;

pub type NodeId = u32;

/// A type representing the scene graph.
/// Nodes are stored in a flat store with "pointers" to their parent (if they have one)
/// and their children.
/// The might be a bit more internal maintenance. But a lot of that should be hidden.
/// On the flip side it is MUCH easier and quicker to just cycle the whole graph without recursion,
/// build indexes etc.#
/// This way the scene graph node also becomes a really simple collection of properties and "pointers".
/// Which should hopefully be a bit more memory efficient and cache friendly.
pub struct SceneGraph {
    nodes: Vec<SceneGraphNode>,
}

pub struct SceneGraphNode {
    active: bool,
    transform_internal_id: TransformInternalId,
    parent_node_id: Option<NodeId>,
    children: Vec<NodeId>,
}

pub struct SceneLoader {
    template: SceneTemplate,
}

/// All the transforms in a scene are kept in flat collection.
/// This is mainly so cycling through them is easy and quick.
/// It is easier to have a none nested/non-hiearchical structure with a hierarchical abstraction on top
/// than then other way round.
/// These are high level transforms rather than finder level ones like per mesh primitive.
/// This idea is this is meant to be used for quick look-ups, tests etc. for a scene,
/// rather than representing something like a model.
/// It does require keeping a list of "pointers" and look-ups for values, but these values should be pretty stable while a scene is loaded.
/// In the future if dynamic loading and unloading of models and resources becomes a thing this could become a pain.
/// However, anything that might need to spawn things at runtime might be better handled with object pooling.
/// That way they still exist here but just won't be queried until active again.
pub struct TransformsCollection {
    root: Transform,
    transforms: Vec<Transform>,
}

type RenderableId = u32;

impl SceneInstance {
    pub fn create(nodes: Vec<SceneGraphNode>, transforms: Vec<Transform>, world_root: Transform, main_camera: Camera) -> Self {
        Self {
            main_camera,
            graph: SceneGraph { nodes },
            transforms: TransformsCollection {
                root: world_root,
                transforms,
            },
        }
    }

    /// Create a blank scene.
    /// This differs from `.new()` because its intention is to always just create the minimum needed scene to prevent crashes.
    /// It doesn't need to actually be able to do anything.
    pub fn blank() -> Self {
        Self {
            main_camera: Camera::create(800. ,600.),
            graph: SceneGraph { nodes: vec![] },
            transforms: TransformsCollection::new(),
        }
    }

    /// The method currently does nothing.
    /// It exists in case in the future scene instances have some resources that need freeing.
    pub fn free(&self, gl: &Context) {}

    //pub fn get_renderable_objects(&self) -> &Vec<RenderableId> {
    //    &self.renderable_objects
    //}

    //pub fn get_renderable_object(&self, internal_id: &u32) -> Option<&u32> {
    //    self.renderable_objects.get(*internal_id as usize)
    //}
}

impl SceneLoader {
    pub fn create(template: SceneTemplate) -> Self {
        Self { template }
    }

    pub fn load_scene_render_resources(&self, gl: &Context) -> NewRendererResources {
        let mut shaders: Vec<Shader> = Vec::with_capacity(self.template.shaders.len());
        let mut textures: Vec<Texture> = Vec::with_capacity(self.template.textures.len());
        let mut materials: Vec<Material> = Vec::with_capacity(self.template.materials.len());
        let mut shaders_map: InternalIdMap = InternalIdMap::new();
        let mut textures_map: InternalIdMap = InternalIdMap::new();
        let mut materials_map: InternalIdMap = InternalIdMap::new();

        let mut shader_internal_id = 0;

        for x in &self.template.shaders {
            let shader = Shader::create(gl, &x.vertex_code, &x.fragment_code);
            shaders.push(shader);
            shaders_map.add(&x.id, shader_internal_id);
            shader_internal_id = shader_internal_id + 1;
        }

        let mut texture_internal_id = 0;

        for x in &self.template.textures {
            let texture = Texture::create(gl, x.data.as_slice(), x.width as i32, x.height as i32);
            textures.push(texture);
            textures_map.add(&x.id, texture_internal_id);
            texture_internal_id += 1;
        }

        let mut material_internal_id = 0;

        for x in &self.template.materials {
            let material = match &x.material_type {
                MaterialTemplateType::Basic(bt) => Material::Basic(BasicMaterial {
                    shader_internal_id: shaders_map.get_internal_id(&bt.shader_id).unwrap(),
                    is_transparent: bt.is_transparent,
                }),
                MaterialTemplateType::Unlit(ut) => Material::Unlit(UnlitMaterial {
                    shader_internal_id: 0,
                    texture_internal_id: 0,
                    is_transparent: ut.is_transparent,
                }),
            };

            materials.push(material);
            materials_map.add(&x.id, material_internal_id);
            material_internal_id = material_internal_id + 1;
        }

        NewRendererResources {
            shaders,
            textures,
            materials,
            shaders_map,
            textures_map,
            materials_map,
        }
    }

    pub fn load_scene_models(
        &self,
        gl: &Context,
        material_id_map: &InternalIdMap,
    ) -> NewModelStoreResources {
        let mut models: Vec<Model> = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();
        let mut primitives: Vec<MeshPrimitive> = Vec::new();
        let mut models_id_map = InternalIdMap::new();
        let mut meshes_id_map = InternalIdMap::new();
        let mut primitives_id_map = InternalIdMap::new();
        let mut next_model_id = 0;
        let mut next_mesh_id = 0;
        let mut next_primitive_id = 0;

        let mut mesh_ids: Vec<MeshInternalId> = Vec::new();
        let mut primitive_ids: Vec<MeshPrimitiveInternalId> = Vec::new();

        for model in &self.template.models {
            //mesh_ids.clear();
            for mesh in &model.meshes {
                //primitive_ids.clear();

                for prim in &mesh.primitives {
                    let material_internal_id =
                        material_id_map.get_internal_id(&prim.material_id).unwrap();

                    let new_primitive = MeshPrimitive::build(
                        gl,
                        &prim.vertices,
                        &next_primitive_id,
                        &next_model_id,
                        &next_mesh_id,
                        &material_internal_id,
                        &prim.local_transform,
                    );

                    primitives.push(new_primitive);
                    primitives_id_map.add(&prim.id, next_primitive_id);
                    primitive_ids.push(next_primitive_id);

                    next_primitive_id = next_primitive_id + 1
                }

                let new_mesh = Mesh::create(
                    &next_mesh_id,
                    &next_primitive_id,
                    mem::take(&mut primitive_ids),
                    &mesh.local_transform,
                );

                meshes.push(new_mesh);
                mesh_ids.push(next_mesh_id);
                meshes_id_map.add(&mesh.id, next_mesh_id);

                next_mesh_id = next_mesh_id + 1
            }

            let new_model = Model::create(
                &next_model_id,
                mem::take(&mut mesh_ids),
                &model.local_transform,
            );

            models.push(new_model);
            models_id_map.add(&model.id, next_model_id);
            next_model_id = next_model_id + 1;
        }

        NewModelStoreResources {
            models,
            meshes,
            primitives,
            models_id_map,
            meshes_id_map,
            primitives_id_map,
        }
    }

    pub fn set_template(&mut self, template: SceneTemplate) {
        self.template = template
    }

    pub fn load_shaders(&self, gl: &Context) -> Vec<Shader> {
        let mut result: Vec<Shader> = Vec::with_capacity(self.template.shaders.len());

        for x in &self.template.shaders {
            let shader = Shader::create(gl, &x.fragment_code, &x.vertex_code);
            result.push(shader)
        }

        result
    }

    pub fn load_textures(&mut self, gl: &Context) -> Vec<Texture> {
        let mut result: Vec<Texture> = Vec::with_capacity(self.template.textures.len());

        for x in &self.template.textures {
            let texture = Texture::create(gl, x.data.as_slice(), x.width as i32, x.height as i32);
            result.push(texture)
        }

        result
    }

    pub fn load_materials(&mut self, gl: &Context) -> Vec<Material> {
        let mut result: Vec<Material> = Vec::with_capacity(self.template.materials.len());

        for x in &self.template.materials {
            let material = match &x.material_type {
                MaterialTemplateType::Basic(bt) => Material::Basic(BasicMaterial {
                    shader_internal_id: 0,
                    is_transparent: bt.is_transparent,
                }),
                MaterialTemplateType::Unlit(ut) => Material::Unlit(UnlitMaterial {
                    shader_internal_id: 0,
                    texture_internal_id: 0,
                    is_transparent: ut.is_transparent,
                }),
            };

            result.push(material);
        }

        result
    }

    pub fn build_scene_instance(&self, display_width: f32, display_height: f32) -> SceneInstance {
        let mut nodes: Vec<SceneGraphNode> = Vec::new();
        let mut transforms: Vec<Transform> = Vec::new();
        let mut main_camera = Camera::create(display_width, display_height);
        //main_camera.yaw = -std::f32::consts::FRAC_PI_2;
        //main_camera.
        main_camera.update_basis();

        let mut next_transform_id = 0;

        // The transforms stored for models are top level.
        // This is because in general you will probably want to move/rotate the whole model most the time.
        for model in &self.template.models {
            transforms.push(model.world_transform.clone());

            nodes.push(SceneGraphNode {
                active: true,
                transform_internal_id: next_transform_id,
                parent_node_id: None,
                children: vec![],
            });

            next_transform_id = next_transform_id + 1;
        }

        SceneInstance::create(nodes, transforms, Transform::default(), main_camera)
    }
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
        }
    }

    pub fn build(
        shaders_map: &InternalIdMap,
        textures_map: &InternalIdMap,
        materials_map: &InternalIdMap,
        models_map: &InternalIdMap,
        meshes_map: &InternalIdMap,
        mesh_primitives_map: &InternalIdMap,
    ) -> ResourcesMap {
        Self {
            materials_map: materials_map.clone(),
            textures_map: textures_map.clone(),
            shaders_map: shaders_map.clone(),
            models_map: models_map.clone(),
            meshes_map: meshes_map.clone(),
            mesh_primitives_map: mesh_primitives_map.clone(),
        }
    }
}

impl TransformsCollection {
    pub fn new() -> Self {
        Self {
            root: Transform::default(),
            transforms: vec![],
        }
    }
}
