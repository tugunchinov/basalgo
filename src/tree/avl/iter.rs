use crate::tree::avl::AvlTree;
use crate::tree::avl::node::AVLTreeNode;

pub fn get_key_value<K, V>(node: &AVLTreeNode<K, V>) -> (&K, &V) {
    (&node.key, &node.value)
}

pub fn get_key<K, V>(node: &AVLTreeNode<K, V>) -> &K {
    &node.key
}

pub fn get_value<K, V>(node: &AVLTreeNode<K, V>) -> &V {
    &node.value
}

#[cfg(test)]
pub fn get_node<K, V>(node: &AVLTreeNode<K, V>) -> &AVLTreeNode<K, V> {
    node
}

pub struct AvlTreeIterator<'a, K, V, I> {
    next_node: Option<&'a AVLTreeNode<K, V>>,
    get_item_func: fn(&'a AVLTreeNode<K, V>) -> I,
}

pub type AvlTreeKeyValueIterator<'a, K, V> = AvlTreeIterator<'a, K, V, (&'a K, &'a V)>;

pub type AvlTreeKeyIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a K>;

pub type AvlTreeValueIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a V>;

#[cfg(test)]
pub type AvlTreeNodeIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a AVLTreeNode<K, V>>;

impl<'a, K, V, R> AvlTreeIterator<'a, K, V, R> {
    pub fn new(
        root: Option<&'a AVLTreeNode<K, V>>,
        get_item_func: fn(&'a AVLTreeNode<K, V>) -> R,
    ) -> Self {
        let next_node = root.as_ref().map(|root| root.find_leftmost_node());

        Self {
            next_node,
            get_item_func,
        }
    }

    fn find_successor(&self, node: &'a AVLTreeNode<K, V>) -> Option<&'a AVLTreeNode<K, V>> {
        if let Some(right) = &node.right {
            return Some(right.find_leftmost_node());
        }

        let mut current = node;

        let mut parent = unsafe { current.parent.as_ref() };

        while let Some(node) = parent {
            // If we're the right child of our parent, we need to go up again
            if node
                .right
                .as_ref()
                .is_some_and(|right| std::ptr::eq(&**right, current))
            {
                current = node;
                parent = unsafe { node.parent.as_ref() };
            } else {
                return Some(node);
            }
        }

        None
    }
}

impl<K, V, R> Iterator for AvlTreeIterator<'_, K, V, R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next_node?;
        let result = (self.get_item_func)(current);
        self.next_node = self.find_successor(current);

        Some(result)
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for AvlTree<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Self::new();

        for i in iter {
            tree.insert(i.0, i.1);
        }

        tree
    }
}

impl<'a, K, V> IntoIterator for &'a AvlTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = AvlTreeKeyValueIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        AvlTreeKeyValueIterator::new(self.root.as_deref(), get_key_value)
    }
}

pub struct AvlTreeOwnedIterator<K, V> {
    stack: Vec<Box<AVLTreeNode<K, V>>>,
}

impl<K, V> AvlTreeOwnedIterator<K, V> {
    fn new(tree: AvlTree<K, V>) -> Self {
        let mut stack = Vec::with_capacity(tree.size());
        let mut current = tree.root;

        while let Some(mut node) = current {
            node.parent = std::ptr::null_mut();
            current = node.left.take();
            stack.push(node);
        }

        Self { stack }
    }
}

impl<K, V> Iterator for AvlTreeOwnedIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;

        let mut current = node.right.take();
        while let Some(mut node) = current {
            node.parent = std::ptr::null_mut();
            current = node.left.take();
            self.stack.push(node);
        }

        Some((node.key, node.value))
    }
}

impl<K, V> IntoIterator for AvlTree<K, V> {
    type Item = (K, V);
    type IntoIter = AvlTreeOwnedIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        AvlTreeOwnedIterator::new(self)
    }
}
