use uuid::Uuid;
use psionic_engine::maths::Float3;
use psionic_engine::maths::Transform;
use psionic_engine::render_pipeline::MaterialInternalId;
use psionic_engine::rendering::geometry::{
    Vertex, VertexAttribute, VertexAttributesLayout, VertexAttributesLayoutItem, VertexCollection,
};
use psionic_engine::rendering::models::Model;
use psionic_engine::templates::{MeshPrimitiveTemplate, MeshTemplate, ModelTemplate};

pub struct QuadModel {}

impl QuadModel {
    fn vertex_layout() -> VertexAttributesLayout {
        VertexAttributesLayout {
            size: 3,
            items: vec![VertexAttributesLayoutItem {
                count: 3,
                active: true,
            }],
        }
    }

    fn vertices() -> Vec<Vertex> {
        vec![
            Vertex {
                attributes: vec![VertexAttribute::Float3(Float3::new(-1.0, -1.0, 5.0))],
            },
            Vertex {
                attributes: vec![VertexAttribute::Float3(Float3::new( 1.0, -1.0, 5.0))],
            },
            Vertex {
                attributes: vec![VertexAttribute::Float3(Float3::new( 1.0,  1.0, 5.0))],
            },
            Vertex {
                attributes: vec![VertexAttribute::Float3(Float3::new(-1.0,  1.0, 5.0))],
            },
        ]
    }

    fn indices() -> Vec<u32> {
        vec![0, 1, 2, 2, 3, 0]
    }

    pub fn create_model_template(material_id: &Uuid) -> ModelTemplate {
        ModelTemplate {
            id: Uuid::new_v4(),
            meshes: vec![MeshTemplate {
                id: Uuid::new_v4(),
                primitives: vec![MeshPrimitiveTemplate {
                    id: Uuid::new_v4(),
                    vertices: VertexCollection::new(
                        QuadModel::vertex_layout(),
                        QuadModel::vertices(),
                        QuadModel::indices(),
                    ),
                    indices: vec![],
                    local_transform: Transform::default(),
                    material_id: material_id.clone(),
                }],
                local_transform: Transform::default(),
            }],
            local_transform: Transform::default(),
            world_transform: Transform::default(),
        }
    }
}
