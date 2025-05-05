use crate::tree::AVLTree;
use crate::tree::avl::node::AVLTreeNode;

mod comprehensive;
mod unit;

impl<K: Ord, V> AVLTree<K, V> {
    pub fn check_parent_references(&self) -> bool {
        if self.root.is_none() {
            return true;
        }

        if !self.root.as_ref().unwrap().parent.is_null() {
            return false;
        }

        Self::check_node_parent_references(&self.root, std::ptr::null_mut())
    }

    fn check_node_parent_references(
        node: &Option<Box<AVLTreeNode<K, V>>>,
        expected_parent: *mut AVLTreeNode<K, V>,
    ) -> bool {
        match node {
            None => true,
            Some(node_ref) => {
                if !std::ptr::eq(node_ref.parent, expected_parent) {
                    return false;
                }

                let this_node_ptr = &**node_ref as *const _ as *mut _;

                Self::check_node_parent_references(&node_ref.left, this_node_ptr)
                    && Self::check_node_parent_references(&node_ref.right, this_node_ptr)
            }
        }
    }
}
