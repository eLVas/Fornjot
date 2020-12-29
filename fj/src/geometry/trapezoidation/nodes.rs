use std::collections::HashMap;

use super::id::{Ids, RawId};

pub struct Nodes<Branch, Leaf> {
    map: HashMap<RawId, Node<Branch, Leaf>>,
    ids: Ids,
}

impl<Branch, Leaf> Nodes<Branch, Leaf> {
    pub fn new() -> Self {
        Nodes {
            map: HashMap::new(),
            ids: Ids::new(),
        }
    }

    pub fn insert_leaf(&mut self, leaf: Leaf) -> NodeId {
        let id = self.ids.next();

        self.map
            .insert(id, Node::Leaf(LeafNode { parent: None, leaf }));

        NodeId(id)
    }

    pub fn insert_branch(
        &mut self,
        branch: Branch,
        above: &NodeId,
        below: &NodeId,
    ) -> NodeId {
        let id = NodeId(self.ids.next());
        self.insert_branch_internal(&id, branch, None, above, below);
        id
    }

    pub fn replace_child(&mut self, child: &NodeId, new_child: &NodeId) {
        let parent = self.get_mut(child).parent_mut().take();

        *self.get_mut(new_child).parent_mut() = parent;

        if let Some(parent) = parent {
            match self.get_mut(&NodeId(parent)) {
                Node::Branch(branch) if child.0 == branch.above => {
                    branch.above = new_child.0;
                }
                Node::Branch(branch) if child.0 == branch.below => {
                    branch.below = new_child.0;
                }
                Node::Branch(_) => {
                    unreachable!("Parent didn't know about child")
                }
                Node::Leaf(_) => {
                    unreachable!("Parent of a node can't be a leaf")
                }
            }
        }
    }

    pub fn change_leaf_to_branch(
        &mut self,
        id: &NodeId,
        branch: Branch,
        above: &NodeId,
        below: &NodeId,
    ) -> Leaf {
        match self.map.remove(&id.0).unwrap() {
            Node::Branch(_) => panic!("Expected leaf, found branch"),
            Node::Leaf(LeafNode { parent, leaf }) => {
                self.insert_branch_internal(&id, branch, parent, above, below);
                leaf
            }
        }
    }

    /// Return a reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get(&self, id: &NodeId) -> &Node<Branch, Leaf> {
        self.map.get(&id.0).unwrap()
    }

    /// Return a mutable reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get_mut(&mut self, id: &NodeId) -> &mut Node<Branch, Leaf> {
        self.map.get_mut(&id.0).unwrap()
    }

    pub fn parent_of(&self, id: &NodeId) -> Option<(NodeId, Relation)> {
        self.get(id).parent().map(|parent_id| {
            let parent = match self.get(&NodeId(parent_id)) {
                Node::Branch(parent) => parent,
                Node::Leaf(_) => unreachable!("Parent is not a branch"),
            };

            let relation = match id {
                id if id.0 == parent.above => Relation::Above,
                id if id.0 == parent.below => Relation::Below,
                _ => {
                    panic!("Parent doesn't relate to child");
                }
            };
            (NodeId(parent_id), relation)
        })
    }

    pub fn above_of(&self, id: &NodeId) -> NodeId {
        match self.get(id) {
            Node::Branch(BranchNode { above, .. }) => NodeId(*above),
            Node::Leaf(_) => {
                // It would be nicer to enforce this statically, through the use
                // of a branch handle, but for now this will do.
                panic!("Expected branch, got leaf.");
            }
        }
    }

    pub fn below_of(&self, id: &NodeId) -> NodeId {
        match self.get(id) {
            Node::Branch(BranchNode { below, .. }) => NodeId(*below),
            Node::Leaf(_) => {
                // It would be nicer to enforce this statically, through the use
                // of a branch handle, but for now this will do.
                panic!("Expected branch, got leaf.");
            }
        }
    }

    pub fn leafs(&self) -> impl Iterator<Item = (NodeId, &Leaf)> + '_ {
        self.map.iter().filter_map(|(&id, node)| match node {
            Node::Leaf(LeafNode { leaf, .. }) => Some((NodeId(id), leaf)),
            _ => None,
        })
    }

    fn insert_branch_internal(
        &mut self,
        id: &NodeId,
        branch: Branch,
        parent: Option<RawId>,
        above: &NodeId,
        below: &NodeId,
    ) {
        // It would be nicer to verify this statically, through the use of some
        // kind of root node handle, but for now this will do.
        assert!(self.get(above).parent().is_none());
        assert!(self.get(below).parent().is_none());

        self.map.insert(
            id.0,
            Node::Branch(BranchNode {
                parent,
                above: above.0,
                below: below.0,
                branch,
            }),
        );

        // Update parents of the new children
        *self.get_mut(above).parent_mut() = Some(id.0);
        *self.get_mut(below).parent_mut() = Some(id.0);
    }
}

/// Identifies a node
///
/// Since nodes can only be added, never removed, a `NodeId` instance is always
/// going to be valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(RawId);

#[derive(Debug, PartialEq)]
pub enum Node<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(LeafNode<Leaf>),
}

impl<Branch, Leaf> Node<Branch, Leaf> {
    pub fn branch(&self) -> Option<&Branch> {
        match self {
            Self::Branch(BranchNode { branch, .. }) => Some(branch),
            Self::Leaf(_) => None,
        }
    }

    pub fn branch_mut(&mut self) -> Option<&mut Branch> {
        match self {
            Self::Branch(BranchNode { branch, .. }) => Some(branch),
            Self::Leaf(_) => None,
        }
    }

    pub fn leaf(&self) -> Option<&Leaf> {
        match self {
            Self::Branch(_) => None,
            Self::Leaf(LeafNode { leaf, .. }) => Some(leaf),
        }
    }

    pub fn leaf_mut(&mut self) -> Option<&mut Leaf> {
        match self {
            Self::Branch(_) => None,
            Self::Leaf(LeafNode { leaf, .. }) => Some(leaf),
        }
    }

    fn parent(&self) -> &Option<RawId> {
        match self {
            Self::Branch(BranchNode { parent, .. }) => parent,
            Self::Leaf(LeafNode { parent, .. }) => parent,
        }
    }

    fn parent_mut(&mut self) -> &mut Option<RawId> {
        match self {
            Self::Branch(BranchNode { parent, .. }) => parent,
            Self::Leaf(LeafNode { parent, .. }) => parent,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BranchNode<T> {
    parent: Option<RawId>,
    above: RawId,
    below: RawId,
    branch: T,
}

#[derive(Debug, PartialEq)]
pub struct LeafNode<T> {
    parent: Option<RawId>,
    leaf: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    use super::Relation;

    type Nodes = super::Nodes<u8, u8>;

    #[test]
    fn nodes_should_insert_leafs() {
        let mut nodes = Nodes::new();

        let mut leaf = 5;
        let id = nodes.insert_leaf(leaf);

        assert_eq!(nodes.get(&id).leaf().unwrap(), &leaf);
        assert_eq!(nodes.get_mut(&id).leaf_mut().unwrap(), &mut leaf);

        assert_eq!(nodes.parent_of(&id), None);
    }

    #[test]
    fn nodes_should_insert_branches() {
        let mut nodes = Nodes::new();

        let leaf_id_a = nodes.insert_leaf(3);
        let leaf_id_b = nodes.insert_leaf(5);

        let mut branch = 1;
        let id = nodes.insert_branch(branch, &leaf_id_a, &leaf_id_b);

        assert_eq!(nodes.get(&id).branch().unwrap(), &branch);
        assert_eq!(nodes.get_mut(&id).branch_mut().unwrap(), &mut branch);

        assert_eq!(nodes.parent_of(&leaf_id_a), Some((id, Relation::Above)));
        assert_eq!(nodes.parent_of(&leaf_id_b), Some((id, Relation::Below)));

        assert_eq!(nodes.above_of(&id), leaf_id_a);
        assert_eq!(nodes.below_of(&id), leaf_id_b);
    }

    #[test]
    fn nodes_should_assign_new_id_when_adding_nodes() {
        let mut nodes = Nodes::new();

        let id_a = nodes.insert_leaf(5);
        let id_b = nodes.insert_leaf(8);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn nodes_should_return_all_leafs() {
        let mut nodes = Nodes::new();

        let leaf_a = 5;
        let leaf_b = 8;

        let id_a = nodes.insert_leaf(leaf_a);
        let id_b = nodes.insert_leaf(leaf_b);

        let mut saw_a = false;
        let mut saw_b = false;

        for (id, leaf) in nodes.leafs() {
            if id == id_a && leaf == &leaf_a {
                saw_a = true;
            }
            if id == id_b && leaf == &leaf_b {
                saw_b = true;
            }
        }

        assert!(saw_a);
        assert!(saw_b);
    }

    #[test]
    fn nodes_should_change_root_leaf_to_branch() {
        let mut nodes = Nodes::new();

        let leaf_tmp = 3;
        let leaf_a = 5;
        let leaf_b = 8;

        let id_branch = nodes.insert_leaf(leaf_tmp);
        let id_leaf_a = nodes.insert_leaf(leaf_a);
        let id_leaf_b = nodes.insert_leaf(leaf_b);

        let mut branch = 1;
        let replaced_leaf = nodes
            .change_leaf_to_branch(&id_branch, branch, &id_leaf_a, &id_leaf_b);

        assert_eq!(replaced_leaf, leaf_tmp);

        assert_eq!(nodes.get(&id_branch).branch().unwrap(), &branch);
        assert_eq!(
            nodes.get_mut(&id_branch).branch_mut().unwrap(),
            &mut branch
        );

        assert_eq!(nodes.parent_of(&id_branch), None);
        assert_eq!(
            nodes.parent_of(&id_leaf_a),
            Some((id_branch, Relation::Above))
        );
        assert_eq!(
            nodes.parent_of(&id_leaf_b),
            Some((id_branch, Relation::Below))
        );
    }

    #[test]
    fn nodes_should_change_non_root_leaf_to_branch() {
        let mut nodes = Nodes::new();

        // Create non-root leaf nodes.
        let root_id = nodes.insert_leaf(3);
        let leaf_id_a = nodes.insert_leaf(5);
        let leaf_id_b = nodes.insert_leaf(8);
        nodes.change_leaf_to_branch(&root_id, 1, &leaf_id_a, &leaf_id_b);

        let non_root_leaf_id = leaf_id_a;

        // Change a non-root leaf into a branch
        let leaf_id_a = nodes.insert_leaf(13);
        let leaf_id_b = nodes.insert_leaf(21);
        nodes.change_leaf_to_branch(
            &non_root_leaf_id,
            2,
            &leaf_id_a,
            &leaf_id_b,
        );

        assert_eq!(
            nodes.parent_of(&non_root_leaf_id),
            Some((root_id, Relation::Above))
        );
    }

    #[test]
    fn nodes_should_replace_children() {
        let mut nodes = Nodes::new();

        // Create nodes with a parent
        let above_id = nodes.insert_leaf(3);
        let below_id = nodes.insert_leaf(5);
        let parent_id = nodes.insert_branch(1, &above_id, &below_id);

        // Create new nodes that will replace the children
        let above_new_id = nodes.insert_leaf(8);
        let below_new_id = nodes.insert_leaf(13);

        nodes.replace_child(&above_id, &above_new_id);
        assert_eq!(
            nodes.parent_of(&above_new_id),
            Some((parent_id, Relation::Above))
        );
        assert_eq!(nodes.above_of(&parent_id), above_new_id);

        nodes.replace_child(&below_id, &below_new_id);
        assert_eq!(
            nodes.parent_of(&below_new_id),
            Some((parent_id, Relation::Below))
        );
        assert_eq!(nodes.below_of(&parent_id), below_new_id);

        assert!(nodes.parent_of(&above_id).is_none());
        assert!(nodes.parent_of(&below_id).is_none());
    }
}
