use crate::tree::avl::AVLTree;
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

impl<K: Ord, V> FromIterator<(K, V)> for AVLTree<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Self::new();

        for i in iter {
            tree.insert(i.0, i.1);
        }

        tree
    }
}

impl<'a, K, V> IntoIterator for &'a AVLTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = AvlTreeKeyValueIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        AvlTreeKeyValueIterator::new(self.root.as_deref(), get_key_value)
    }
}
