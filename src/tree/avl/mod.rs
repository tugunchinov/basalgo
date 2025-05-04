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
                self.update_heights_and_rebalance(parent_node, 0);
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

    pub fn contains<Q: Borrow<K>>(&self, key: &Q) -> bool {
        self.get(key).is_some()
    }

    pub fn remove<Q: Borrow<K>>(&mut self, key: &Q) -> Option<V> {
        let to_remove_opt = {
            let mut current = &mut self.root;
            while current.is_some() {
                match key.borrow().cmp(&current.as_ref().unwrap().key) {
                    std::cmp::Ordering::Less => current = &mut current.as_mut().unwrap().left,
                    std::cmp::Ordering::Greater => current = &mut current.as_mut().unwrap().right,
                    std::cmp::Ordering::Equal => break,
                }
            }

            current.take()
        };

        if let Some(to_remove) = to_remove_opt {
            let removed = self.remove_node(to_remove);
            self.size -= 1;
            Some(removed.value)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
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

impl<K, V> AVLTree<K, V> {
    fn update_heights_and_rebalance(&mut self, from_node: &mut AVLTreeNode<K, V>, stop_factor: i8) {
        let mut current_node = from_node;
        loop {
            current_node.update_height();

            if current_node.balance_factor().abs() == stop_factor {
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

                if current_node.balance_factor().abs() == stop_factor {
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

    fn remove_node(&mut self, mut node: Box<AVLTreeNode<K, V>>) -> Box<AVLTreeNode<K, V>> {
        let parent_node = unsafe { node.parent.as_mut() };

        if node.left.is_none() && node.right.is_none() {
            if let Some(parent_node) = parent_node {
                // TODO: can't check parent: it's taken!!!!!!!!!!!!!!!!!

                parent_node.replace_child(&mut node, None);
                self.update_heights_and_rebalance(parent_node, 1);
            } else {
                return node;
            }
        } else if node.left.is_some() && node.right.is_some() {
            let successor_ref = node.right.as_ref().unwrap().find_leftmost_node();
            let successor_parent = successor_ref.parent;

            let successor_node = self
                .get_mutable_node_reference(successor_ref)
                .take()
                .unwrap();

            // swap nodes

            let mut successor_ref;
            if let Some(parent_node) = parent_node {
                successor_ref = parent_node.replace_child(&mut node, Some(successor_node));
            } else {
                self.root = Some(successor_node);
                successor_ref = self.root.as_mut().unwrap();
                successor_ref.parent = std::ptr::null_mut();
            };

            successor_ref.left = node.left.take();

            if std::ptr::eq(&**node.right.as_ref().unwrap(), successor_ref) {
                node.right = None;
            } else {
                std::mem::swap(&mut node.right, &mut successor_ref.right);

                let replacing_node = {
                    let successor_parent = unsafe { &mut *successor_parent };
                    successor_parent.left = node.right.take();
                    successor_parent.left.as_mut()
                };

                if let Some(node) = replacing_node {
                    node.parent = successor_parent;
                }
            }

            let successor_parent = unsafe { &mut *successor_parent };
            self.update_heights_and_rebalance(successor_parent, 1);
        } else {
            let mut child = if node.left.is_some() {
                node.left.take().unwrap()
            } else {
                node.right.take().unwrap()
            };

            if let Some(parent_node) = parent_node {
                parent_node.replace_child(&mut node, Some(child));
                self.update_heights_and_rebalance(parent_node, 1);
            } else {
                child.parent = std::ptr::null_mut();
                self.root = Some(child);
                return node;
            }
        }

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

    fn nodes(&self) -> AvlTreeNodeIterator<K, V> {
        AvlTreeNodeIterator::new(self.root.as_deref(), get_node)
    }
}

impl<K: Ord, V> Default for AVLTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
