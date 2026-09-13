
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
    pub active: bool,
    pub transform_internal_id: TransformInternalId,
    pub parent_node_id: Option<NodeId>,
    pub children: Vec<NodeId>,
}