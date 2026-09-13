use crate::camera::Camera;
use crate::core::InternalIdMap;
use crate::maths::Transform;
use crate::rendering::geometry::{ElementsRenderableObject, RenderableObject};
use crate::rendering::materials::{BasicMaterial, Material, UnlitMaterial};
use crate::rendering::models::{
    Mesh, MeshInternalId, MeshPrimitive, MeshPrimitiveInternalId, Model,
};
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use crate::resources::resource_manager::{NewResourcesCollection, PreviousResourcesCollection};
use crate::resources::resources_map::ResourcesMap;
use crate::scenes::scene_graph::{NodeId, SceneGraph, SceneGraphNodeRenderable};
use crate::scenes::{SceneGraphNode, SceneInstance, TransformsCollection};
use crate::templates::{
    MaterialTemplateType, ModelTemplate, SceneGraphNodeTemplate, SceneGraphNodeTemplateType,
    SceneTemplate,
};
use glow::Context;
use std::collections::HashMap;
use std::mem;
use uuid::Uuid;

pub struct SceneLoader {
    template: SceneTemplate,
}

pub struct LoadedScene {
    pub scene_instance: SceneInstance,
    pub resources: NewResourcesCollection,
}

pub struct PreviousScene {
    pub scene_instance: SceneInstance,
    pub resources: PreviousResourcesCollection,
}

impl SceneLoader {
    pub fn create(template: SceneTemplate) -> Self {
        let mut models_map = HashMap::new();

        for model in &template.models {
            models_map.insert(model.id, model);
        }

        Self {
            template,
        }
    }

    pub fn load_scene(&self, gl: &Context, display_width: f32, display_height: f32) -> LoadedScene {
        let new_resources = self.load_resources(gl);
        let new_scene =
            self.build_scene_instance(display_width, display_height, &new_resources.resource_map);
        LoadedScene {
            scene_instance: new_scene,
            resources: new_resources,
        }
    }

    pub fn load_resources(&self, gl: &Context) -> NewResourcesCollection {
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
                    shader_internal_id: shaders_map.get_internal_id(&ut.shader_id).unwrap(),
                    texture_internal_id: textures_map.get_internal_id(&ut.texture_id).unwrap(),
                    is_transparent: ut.is_transparent,
                }),
            };

            materials.push(material);
            materials_map.add(&x.id, material_internal_id);
            material_internal_id = material_internal_id + 1;
        }

        // Models

        let mut models: Vec<Model> = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();
        let mut primitives: Vec<MeshPrimitive> = Vec::new();
        let mut models_map = InternalIdMap::new();
        let mut meshes_map = InternalIdMap::new();
        let mut mesh_primitives_map = InternalIdMap::new();
        let mut next_model_id = 0;
        let mut next_mesh_id = 0;
        let mut next_primitive_id = 0;

        let mut mesh_ids: Vec<MeshInternalId> = Vec::new();
        let mut primitive_ids: Vec<MeshPrimitiveInternalId> = Vec::new();

        let mut renderable_objects = Vec::new();
        let mut renderable_objects_map = InternalIdMap::new();

        let mut next_renderable_object_id = 0;

        for model in &self.template.models {
            //mesh_ids.clear();
            for mesh in &model.meshes {
                //primitive_ids.clear();

                for prim in &mesh.primitives {
                    let material_internal_id =
                        materials_map.get_internal_id(&prim.material_id).unwrap();

                    let new_primitive = MeshPrimitive::create(
                        &prim.vertices,
                        &next_primitive_id,
                        &next_model_id,
                        &next_mesh_id,
                        &material_internal_id,
                        &prim.local_transform,
                    );

                    // Bit cheeky perhaps, but make the renderable object from the primitive.
                    // Then add it to the list.
                    // It is done here so it can easily share the same id as the primitive.
                    let em = ElementsRenderableObject::from_mesh_primitive(&gl, &new_primitive);

                    primitives.push(new_primitive);
                    mesh_primitives_map.add(&prim.id, next_primitive_id);
                    primitive_ids.push(next_primitive_id);

                    next_primitive_id = next_primitive_id + 1;

                    // Create renderable object.

                    // ASSUMPTION - primitive id is the same as the renderable object id.
                    renderable_objects_map.add(&prim.id, next_renderable_object_id);

                    renderable_objects.push(RenderableObject::Elements(em));
                    next_renderable_object_id = next_renderable_object_id + 1;
                }

                let new_mesh = Mesh::create(
                    &next_mesh_id,
                    &next_primitive_id,
                    mem::take(&mut primitive_ids),
                    &mesh.local_transform,
                );

                meshes.push(new_mesh);
                mesh_ids.push(next_mesh_id);
                meshes_map.add(&mesh.id, next_mesh_id);

                next_mesh_id = next_mesh_id + 1
            }

            let new_model = Model::create(
                &next_model_id,
                mem::take(&mut mesh_ids),
                &model.local_transform,
            );

            models.push(new_model);
            models_map.add(&model.id, next_model_id);
            next_model_id = next_model_id + 1;
        }

        NewResourcesCollection {
            renderable_objects,
            textures,
            shaders,
            materials,
            models,
            meshes,
            primitives,
            resource_map: ResourcesMap {
                materials_map,
                textures_map,
                shaders_map,
                models_map,
                meshes_map,
                mesh_primitives_map,
                // TODO - is a map actually needed?
                renderable_objects_map,
            },
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

    pub fn load_materials(&mut self, _gl: &Context) -> Vec<Material> {
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

    fn build_node(
        &self,
        resources_map: &ResourcesMap,
        node: &SceneGraphNodeTemplate,
        current_transform_id: u32,
        current_node_id: u32,
        transforms: &mut Vec<Transform>,
        nodes: &mut Vec<SceneGraphNode>,
        parent_internal_id: Option<NodeId>,
    ) -> (u32, u32) {
        let mut renderables = Vec::new();

        let mut next_transform_id = current_transform_id;
        let mut next_node_id = current_node_id;

        match node.template_type {
            SceneGraphNodeTemplateType::Empty => {},
            SceneGraphNodeTemplateType::Model(model_id) => {
                match resources_map.models_map.get_internal_id(&model_id) {
                    Some(model) => {
                        let model_template = self.template.models.get(model as usize).unwrap();

                        for mesh in &model_template.meshes {
                            for primitive in &mesh.primitives {
                                match (
                                    resources_map
                                        .renderable_objects_map
                                        .get_internal_id(&primitive.id),
                                    resources_map
                                        .materials_map
                                        .get_internal_id(&primitive.material_id),
                                ) {
                                    (None, _) => {}
                                    (_, None) => {}
                                    (Some(ro_id), Some(m_id)) => {
                                        let renderable =
                                            SceneGraphNodeRenderable::create(ro_id, m_id);
                                        renderables.push(renderable);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        panic!("Model with id {} not found in resource map", model_id);
                    }
                }
            }
        }

        transforms.push(node.transform.clone());
        next_transform_id += 1;

        let mut children = Vec::new();

        for child in &node.children {
            let (nni, nti) = self.build_node(
                &resources_map,
                child,
                next_node_id,
                next_transform_id,
                transforms,
                nodes,
                Some(next_node_id),
            );
            next_node_id = nni;
            next_transform_id = nti;
            children.push(nni);
        }

        transforms.push(node.transform.clone());

        nodes.push(SceneGraphNode {
            id: next_node_id,
            active: true,
            transform_internal_id: next_transform_id,
            parent_node_id: parent_internal_id,
            children,
            renderables,
        });

        (next_node_id, next_transform_id)
    }

    /// Builds the scene internals.
    /// This includes building the scene graph and the transforms collection.
    /// It returns them as a tuple, not the prettiest.
    /// That is why it is internal.
    fn build_scene_internals(
        &self,
        resource_map: &ResourcesMap,
    ) -> (SceneGraph, TransformsCollection) {
        let mut nodes = Vec::new();
        let mut transforms: Vec<Transform> = Vec::new();

        let mut next_transform_id = 0;
        let mut next_node_id = 0;

        for node in &self.template.scene_graph_template.root_node.children {
            //transforms.push(node.transform.clone());

            let (nni, nti) = self.build_node(
                resource_map,
                node,
                next_transform_id,
                next_node_id,
                &mut transforms,
                &mut nodes,
                None,
            );

            next_node_id = nni;
            next_transform_id = nti;

            /*
            let mut renderables = Vec::new();

            match node.template_type {
                SceneGraphNodeTemplateType::Model(model_id) => {
                    match resource_map.models_map.get_internal_id(&model_id) {
                        Some(model) => {
                            let model_template = self.template.models.get(model as usize).unwrap();

                            for mesh in &model_template.meshes {
                                for primitive in &mesh.primitives {
                                    match (
                                        resource_map
                                            .renderable_objects_map
                                            .get_internal_id(&primitive.id),
                                        resource_map
                                            .materials_map
                                            .get_internal_id(&primitive.material_id),
                                    ) {
                                        (None, _) => {}
                                        (_, None) => {}
                                        (Some(ro_id), Some(m_id)) => {
                                            let renderable =
                                                SceneGraphNodeRenderable::create(ro_id, m_id);
                                            renderables.push(renderable);
                                        }
                                    }
                                }
                            }

                            transforms.push(node.transform.clone());
                            next_transform_id += 1;
                        }
                        None => {
                            panic!("Model with id {} not found in resource map", model_id);
                        }
                    }
                }
            }

            transforms.push(node.transform.clone());

            nodes.push(SceneGraphNode {
                id: next_node_id,
                active: true,
                transform_internal_id: next_transform_id,
                parent_node_id: None,
                children: vec![],
                renderables,
            });

            next_node_id += 1;
            next_transform_id += 1;
            */
        }

        (
            SceneGraph { nodes },
            TransformsCollection {
                root: Transform::default(),
                transforms,
            },
        )
    }

    pub fn build_scene_instance(
        &self,
        display_width: f32,
        display_height: f32,
        resource_map: &ResourcesMap,
    ) -> SceneInstance {
        //let mut nodes: Vec<SceneGraphNode> = Vec::new();
        let mut transforms: Vec<Transform> = Vec::new();

        let mut main_camera = Camera::create(display_width, display_height);
        main_camera.initialize(&self.template.main_camera_settings);

        //main_camera.yaw = -std::f32::consts::FRAC_PI_2;

        let mut next_transform_id = 0;

        let mut next_node_id = 0;

        let (graph, transforms) = self.build_scene_internals(resource_map);
        //
        //self.template.scene_graph_template.root_node.build_scene_graph_nodes();

        /*
        // The transforms stored for models are top level.
        // This is because in general you will probably want to move/rotate the whole model most the time.
        for model in &self.template.models {
            transforms.push(model.world_transform.clone());

            let mut renderables = Vec::new();

            for mesh in &model.meshes {
                for primitive in &mesh.primitives {
                    let renderable_obj_id = resource_map
                        .renderable_objects_map
                        .get_internal_id(&primitive.id);
                    let material_id = resource_map
                        .materials_map
                        .get_internal_id(&primitive.material_id);

                    // ASSUMPTION - primitive id is the same as the renderable object id.
                    match (renderable_obj_id, material_id) {
                        (None, _) => {
                            println!("No renderable object id for primitive: {}", &primitive.id);
                        }
                        (_, None) => {
                            println!("No material id for primitive: {}", &primitive.material_id);
                        }
                        (Some(ro_id), Some(m_id)) => {
                            let renderable = SceneGraphNodeRenderable::create(ro_id, m_id);
                            renderables.push(renderable);
                        }
                    }
                }
            }

            //

            // TODO parents and children
            nodes.push(SceneGraphNode {
                id: next_node_id,
                active: true,
                transform_internal_id: next_transform_id,
                parent_node_id: None,
                children: vec![],
                renderables,
            });

            next_transform_id = next_transform_id + 1;
            next_node_id = next_node_id + 1;
        }
        */

        SceneInstance::create(graph, transforms, main_camera)
    }
}
