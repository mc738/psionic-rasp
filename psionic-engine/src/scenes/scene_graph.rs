use crate::render_pipeline::MaterialInternalId;
use crate::rendering::geometry::RenderableObjectInternalId;
use crate::resources::{ResourceLink, ResourceLinks};

pub type TransformInternalId = u32;

pub type NodeId = u32;

/// A type representing the scene graph.
/// Nodes are stored in a flat store with "pointers" to their parent (if they have one)
/// and their children.
/// They might be a bit more internal maintenance. But a lot of that should be hidden.
/// On the flip side it is MUCH easier and quicker to just cycle the whole graph without recursion
/// when build indexes etc.
/// This way the scene graph node also becomes a really simple collection of properties and "pointers".
/// Which should hopefully be a bit more memory efficient and cache friendly.
pub struct SceneGraph {
    pub nodes: Vec<SceneGraphNode>,
}

pub struct SceneGraphNode {
    pub id: NodeId,
    pub active: bool,
    pub transform_internal_id: TransformInternalId,
    pub parent_node_id: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub renderables: Vec<SceneGraphNodeRenderable>,
    //pub resource_links: ResourceLinks,
}

pub struct SceneGraphNodeRenderable {
    pub renderable_object_internal_id: RenderableObjectInternalId,
    pub material_internal_id: MaterialInternalId,
}

impl SceneGraph {

    /*
    pub fn get_renderable_nodes(&self) -> &[&SceneGraphNodeRenderable] {
        let mut renderable_nodes = Vec::new();

        for node in &self.nodes {
            if self.is_active(node.id) && node.renderables.len() > 0 {

                for renderable in &node.renderables {
                    renderable_nodes.push(renderable);
                }
            }
        }

        renderable_nodes.as_slice()
    }
    */

    pub fn is_active(&self, node_id: NodeId) -> bool {
        if self.nodes[node_id as usize].active {
            let mut parent = self.nodes[node_id as usize].parent_node_id;

            while let Some(parent_node_id) = parent {
                if !self.nodes[parent_node_id as usize].active {
                    return false;
                }
                parent = self.nodes[parent_node_id as usize].parent_node_id;
            }

            true
        }
        else
        {
            false
        }
    }
}

impl SceneGraphNodeRenderable {
    pub fn create(renderable_object_internal_id: RenderableObjectInternalId, material_internal_id: MaterialInternalId) -> SceneGraphNodeRenderable {
        SceneGraphNodeRenderable {
            renderable_object_internal_id,
            material_internal_id,
        }
    }
}