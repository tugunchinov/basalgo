mod iter;
mod node;

#[cfg(test)]
mod tests;

use crate::tree::avl::iter::{
    AvlTreeKeyIterator, AvlTreeKeyValueIterator, AvlTreeValueIterator, get_key, get_value,
};
use crate::tree::avl::node::AVLTreeNode;
use std::borrow::Borrow;

#[cfg(test)]
use crate::tree::avl::iter::AvlTreeNodeIterator;
#[cfg(test)]
use crate::tree::avl::iter::get_node;

pub struct AvlTree<K, V> {
    root: Option<Box<AVLTreeNode<K, V>>>,
    size: usize,
}

impl<K: Ord, V> AvlTree<K, V> {
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
            parent = &mut **node;

            match key.cmp(&node.key) {
                std::cmp::Ordering::Less => current_node = &mut node.left,
                std::cmp::Ordering::Greater => current_node = &mut node.right,
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
                let parent_node = unsafe { &mut *parent };
                self.update_heights_and_rebalance(parent_node, 0);
            }
        }

        self.size += 1;

        None
    }

    pub fn get<Q: Borrow<K>>(&self, key: &Q) -> Option<&V> {
        let mut current_node = self.root.as_ref()?;

        loop {
            match key.borrow().cmp(&current_node.key) {
                std::cmp::Ordering::Less => current_node = current_node.left.as_ref()?,
                std::cmp::Ordering::Greater => current_node = current_node.right.as_ref()?,
                std::cmp::Ordering::Equal => return Some(&current_node.value),
            }
        }
    }

    pub fn contains<Q: Borrow<K>>(&self, key: &Q) -> bool {
        self.get(key).is_some()
    }

    pub fn remove<Q: Borrow<K>>(&mut self, key: &Q) -> Option<V> {
        let (to_remove, node_type) = {
            let mut node_type = NodeType::Root;
            let mut current = &mut self.root;
            while current.is_some() {
                match key.borrow().cmp(&current.as_ref().unwrap().key) {
                    std::cmp::Ordering::Less => {
                        current = &mut current.as_mut().unwrap().left;
                        node_type = NodeType::LeftChild;
                    }
                    std::cmp::Ordering::Greater => {
                        current = &mut current.as_mut().unwrap().right;
                        node_type = NodeType::RightChild;
                    }
                    std::cmp::Ordering::Equal => break,
                }
            }

            (current.take()?, node_type)
        };

        self.size -= 1;

        if to_remove.left.is_none() && to_remove.right.is_none() {
            Some(self.remove_leaf_node(to_remove).value)
        } else if to_remove.left.is_some() && to_remove.right.is_some() {
            Some(self.remove_two_children_node(to_remove, node_type).value)
        } else {
            Some(self.remove_one_child_node(to_remove, node_type).value)
        }
    }

    pub fn min(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|root| {
            let node = root.find_leftmost_node();
            (&node.key, &node.value)
        })
    }

    pub fn max(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|root| {
            let node = root.find_rightmost_node();
            (&node.key, &node.value)
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

impl<K, V> AvlTree<K, V> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn update_heights_and_rebalance(
        &mut self,
        from_node: &mut AVLTreeNode<K, V>,
        _stop_factor: i8,
    ) {
        let mut current_node = from_node;
        loop {
            current_node.update_height();

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
            }

            if let Some(parent_node) = unsafe { current_node.parent.as_mut() } {
                current_node = parent_node;
            } else {
                break;
            }
        }
    }

    fn remove_leaf_node(&mut self, mut node: Box<AVLTreeNode<K, V>>) -> Box<AVLTreeNode<K, V>> {
        let parent_node = unsafe { node.parent.as_mut() };

        if let Some(parent_node) = parent_node {
            self.update_heights_and_rebalance(parent_node, 1);
        }

        node.parent = std::ptr::null_mut();
        node
    }

    fn remove_one_child_node(
        &mut self,
        mut node: Box<AVLTreeNode<K, V>>,
        node_type: NodeType,
    ) -> Box<AVLTreeNode<K, V>> {
        let parent_node = unsafe { node.parent.as_mut() };

        let mut child = if node.left.is_some() {
            node.left.take().unwrap()
        } else {
            node.right.take().unwrap()
        };

        if let Some(parent_node) = parent_node {
            child.parent = parent_node;

            match node_type {
                NodeType::LeftChild => parent_node.left = Some(child),
                NodeType::RightChild => parent_node.right = Some(child),
                NodeType::Root => unreachable!(),
            }

            self.update_heights_and_rebalance(parent_node, 1);
        } else {
            child.parent = std::ptr::null_mut();
            self.root = Some(child);
        }

        node.parent = std::ptr::null_mut();
        node
    }

    fn remove_two_children_node(
        &mut self,
        mut node: Box<AVLTreeNode<K, V>>,
        node_type: NodeType,
    ) -> Box<AVLTreeNode<K, V>> {
        let parent_node = unsafe { node.parent.as_mut() };

        let successor_ref = node.right.as_ref().unwrap().find_leftmost_node();
        let successor_parent = successor_ref.parent;

        let mut successor_node = self
            .get_mutable_node_reference(successor_ref)
            .take()
            .unwrap();

        let is_successor_direct_child = node.right.is_none();

        // swap nodes

        successor_node.left = node.left.take().map(|mut node| {
            node.parent = &mut *successor_node;
            node
        });

        if !is_successor_direct_child {
            std::mem::swap(&mut node.right, &mut successor_node.right);
            successor_node.right.as_mut().unwrap().parent = &mut *successor_node;
        }

        let successor_ref = if let Some(parent_node) = parent_node {
            successor_node.parent = parent_node;
            match node_type {
                NodeType::LeftChild => {
                    parent_node.left = Some(successor_node);
                    &mut **parent_node.left.as_mut().unwrap()
                }
                NodeType::RightChild => {
                    parent_node.right = Some(successor_node);
                    &mut **parent_node.right.as_mut().unwrap()
                }
                NodeType::Root => unreachable!(),
            }
        } else {
            successor_node.parent = std::ptr::null_mut();
            self.root = Some(successor_node);
            &mut **self.root.as_mut().unwrap()
        };

        if !is_successor_direct_child {
            let replacing_node = {
                let successor_parent = unsafe { &mut *successor_parent };
                successor_parent.left = node.right.take();
                successor_parent.left.as_mut()
            };

            if let Some(node) = replacing_node {
                node.parent = successor_parent;
            }

            successor_ref.update_height();
            let successor_parent = unsafe { &mut *successor_parent };
            self.update_heights_and_rebalance(successor_parent, 1);
        } else {
            // dirty hack
            let successor_ref = unsafe {
                (successor_ref as *const _ as *mut AVLTreeNode<K, V>)
                    .as_mut()
                    .unwrap()
            };
            self.update_heights_and_rebalance(successor_ref, 1);
        }

        node.parent = std::ptr::null_mut();
        node
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

        if parent.is_left_child(node) {
            return &mut parent.left;
        }

        if parent.is_right_child(node) {
            return &mut parent.right;
        }

        panic!("broken tree");
    }

    #[cfg(test)]
    fn nodes(&self) -> AvlTreeNodeIterator<K, V> {
        AvlTreeNodeIterator::new(self.root.as_deref(), get_node)
    }
}

impl<K: Ord, V> Default for AvlTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

enum NodeType {
    LeftChild,
    RightChild,
    Root,
}
