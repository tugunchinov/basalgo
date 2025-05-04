mod iter;
mod node;

#[cfg(test)]
mod tests;

use crate::tree::avl::iter::{
    AvlTreeKeyIterator, AvlTreeKeyValueIterator, AvlTreeNodeIterator, AvlTreeValueIterator,
    get_key, get_node, get_value,
};
use crate::tree::avl::node::AVLTreeNode;
use std::borrow::Borrow;

pub struct AVLTree<K, V> {
    root: Option<Box<AVLTreeNode<K, V>>>,
    size: usize,
}

impl<K: Ord, V> AVLTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current_node = &mut self.root;
        let mut parent = std::ptr::null_mut();

        while let Some(node) = current_node {
            match key.cmp(&node.key) {
                std::cmp::Ordering::Less => {
                    parent = &mut **node;
                    current_node = &mut node.left;
                }
                std::cmp::Ordering::Greater => {
                    parent = &mut **node;
                    current_node = &mut node.right;
                }
                std::cmp::Ordering::Equal => {
                    let old_value = std::mem::replace(&mut node.value, value);
                    return Some(old_value);
                }
            }
        }

        let mut new_node = AVLTreeNode::new(key, value);
        new_node.parent = parent;
        *current_node = Some(Box::new(new_node));

        if let Some(node) = current_node.as_mut() {
            // Start rebalancing from the parent of the inserted node
            if !node.parent.is_null() {
                let parent_node = unsafe { &mut *node.parent };
                self.update_heights_and_rebalance(parent_node);
            }
        }

        self.size += 1;

        None
    }

    pub fn get<Q: Borrow<K>>(&self, key: &Q) -> Option<&V> {
        let mut current_node = &self.root;

        while let Some(node) = current_node {
            match key.borrow().cmp(&node.key) {
                std::cmp::Ordering::Less => current_node = &node.left,
                std::cmp::Ordering::Greater => current_node = &node.right,
                std::cmp::Ordering::Equal => {
                    return Some(&node.value);
                }
            }
        }

        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|root| {
            let mut current = root;
            while let Some(left) = &current.left {
                current = left;
            }
            (&current.key, &current.value)
        })
    }

    pub fn max(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|root| {
            let mut current = root;
            while let Some(right) = &current.right {
                current = right;
            }
            (&current.key, &current.value)
        })
    }

    pub fn iter(&self) -> AvlTreeKeyValueIterator<K, V> {
        self.into_iter()
    }

    pub fn keys(&self) -> AvlTreeKeyIterator<K, V> {
        AvlTreeKeyIterator::new(self.root.as_deref(), get_key)
    }

    pub fn values(&self) -> AvlTreeValueIterator<K, V> {
        AvlTreeValueIterator::new(self.root.as_deref(), get_value)
    }
}

impl<K, V> AVLTree<K, V> {
    fn update_heights_and_rebalance(&mut self, from_node: &mut AVLTreeNode<K, V>) {
        let mut current_node = from_node;
        loop {
            current_node.update_height();

            if current_node.balance_factor() == 0 {
                return;
            }

            if current_node.balance_factor().abs() >= 2 {
                let current_node_in_tree = self.get_mutable_node_reference(current_node);

                if current_node.balance_factor() == -2 {
                    let right_child_balance_factor = current_node
                        .right
                        .as_ref()
                        .map(|node| node.balance_factor())
                        .unwrap_or(0);

                    if right_child_balance_factor == -1 || right_child_balance_factor == 0 {
                        AVLTreeNode::rotate_left(current_node_in_tree);
                    } else if right_child_balance_factor == 1 {
                        AVLTreeNode::big_rotate_left(current_node_in_tree);
                    }
                } else if current_node.balance_factor() == 2 {
                    let left_child_balance_factor = current_node
                        .left
                        .as_ref()
                        .map(|node| node.balance_factor())
                        .unwrap_or(0);

                    if left_child_balance_factor == 1 || left_child_balance_factor == 0 {
                        AVLTreeNode::rotate_right(current_node_in_tree);
                    } else if left_child_balance_factor == -1 {
                        AVLTreeNode::big_rotate_right(current_node_in_tree);
                    }
                }

                if current_node.balance_factor() == 0 {
                    return;
                }
            }

            if let Some(parent_node) = unsafe { current_node.parent.as_mut() } {
                current_node = parent_node;
            } else {
                break;
            }
        }
    }

    /// Panics if a node is not in the tree
    fn get_mutable_node_reference(
        &mut self,
        node: &AVLTreeNode<K, V>,
    ) -> &mut Option<Box<AVLTreeNode<K, V>>> {
        if node.parent.is_null() {
            // must be the root
            if !self
                .root
                .as_ref()
                .is_some_and(|root| std::ptr::eq(&**root, node))
            {
                panic!("broken tree");
            }

            return &mut self.root;
        }

        let parent = unsafe { &mut *node.parent };

        if let Some(left_ptr) = parent.left.as_deref() {
            if std::ptr::eq(left_ptr, node) {
                return &mut parent.left;
            }
        }

        if let Some(right_ptr) = parent.right.as_deref() {
            if std::ptr::eq(right_ptr, node) {
                return &mut parent.right;
            }
        }

        panic!("broken tree");
    }

    fn nodes(&self) -> AvlTreeNodeIterator<K, V> {
        AvlTreeNodeIterator::new(self.root.as_deref(), get_node)
    }
}

impl<K: Ord, V> Default for AVLTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
