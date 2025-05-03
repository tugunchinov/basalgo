use std::borrow::Borrow;

struct AVLTreeNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<AVLTreeNode<K, V>>>,
    right: Option<Box<AVLTreeNode<K, V>>>,

    /// SAFETY: This pointer lives as long as the node itself.
    /// Used for iterator
    parent: *mut AVLTreeNode<K, V>,

    // TODO:
    // Так как высоты левых и правых поддеревьев в АВЛ-дереве отличаются максимум на 1
    // , то мы будем хранить не всю высоту дерева, а некоторое число, которое будет показывать, какое поддерево больше, или равны ли они, назовём фактор баланса. Таким образом в каждом узле будет храниться 1
    //  — если высота правого поддерева выше левого, 0
    //  — если высоты равны, и −1
    //  — если правое поддерево выше левого.
    height: usize,
}

impl<K, V> AVLTreeNode<K, V> {
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height());
    }

    fn balance_factor(&self) -> i8 {
        (self.left_height() - self.right_height()) as i8
    }

    fn update_ancestors_height(&mut self) {
        self.update_height();

        let mut current_node = self;
        while let Some(parent_node) = unsafe { current_node.parent.as_mut() } {
            parent_node.update_height();

            if parent_node.balance_factor() == 0 {
                return;
            }

            if parent_node.balance_factor().abs() >= 2 {
                // TODO: rebalance
            }

            current_node = parent_node;
        }
    }
}

struct AVLTree<K, V> {
    root: Option<Box<AVLTreeNode<K, V>>>,
}

impl<K: Ord, V> AVLTree<K, V> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current_node = &mut self.root;
        let mut parent = std::ptr::null_mut();

        while let Some(node) = current_node {
            match node.key.cmp(&key) {
                std::cmp::Ordering::Less => {
                    parent = &mut **node;
                    current_node = &mut node.left
                }
                std::cmp::Ordering::Greater => {
                    parent = &mut **node;
                    current_node = &mut node.right
                }
                std::cmp::Ordering::Equal => {
                    let old_value = std::mem::replace(&mut node.value, value);
                    return Some(old_value);
                }
            }
        }

        *current_node = Some(Box::new(AVLTreeNode {
            key,
            value,
            left: None,
            right: None,
            parent,
            height: 0,
        }));

        None
    }

    fn get<Q: Borrow<K>>(&self, key: &Q) -> Option<&V> {
        let mut current_node = &self.root;

        while let Some(node) = current_node {
            match node.key.cmp(key.borrow()) {
                std::cmp::Ordering::Less => current_node = &node.left,
                std::cmp::Ordering::Greater => current_node = &node.right,
                std::cmp::Ordering::Equal => {
                    return Some(&node.value);
                }
            }
        }

        None
    }

    fn iter(&self) -> AvlTreeKeyValueIterator<K, V> {
        self.into_iter()
    }

    fn keys(&self) -> AvlTreeKeyIterator<K, V> {
        AvlTreeKeyIterator::new(self.root.as_deref(), get_key)
    }

    fn values(&self) -> AvlTreeValueIterator<K, V> {
        AvlTreeValueIterator::new(self.root.as_deref(), get_value)
    }

    fn nodes(&self) -> AvlTreeNodeIterator<K, V> {
        AvlTreeNodeIterator::new(self.root.as_deref(), get_node)
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

fn get_key_value<K, V>(node: &AVLTreeNode<K, V>) -> (&K, &V) {
    (&node.key, &node.value)
}

fn get_key<K, V>(node: &AVLTreeNode<K, V>) -> &K {
    &node.key
}

fn get_value<K, V>(node: &AVLTreeNode<K, V>) -> &V {
    &node.value
}

fn get_node<K, V>(node: &AVLTreeNode<K, V>) -> &AVLTreeNode<K, V> {
    node
}

struct AvlTreeIterator<'a, K, V, I> {
    next_node: Option<&'a AVLTreeNode<K, V>>,
    get_item_func: fn(&'a AVLTreeNode<K, V>) -> I,
}

type AvlTreeKeyValueIterator<'a, K, V> = AvlTreeIterator<'a, K, V, (&'a K, &'a V)>;

type AvlTreeKeyIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a K>;

type AvlTreeValueIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a V>;

type AvlTreeNodeIterator<'a, K, V> = AvlTreeIterator<'a, K, V, &'a AVLTreeNode<K, V>>;

impl<'a, K, V, R> AvlTreeIterator<'a, K, V, R> {
    fn new(
        root: Option<&'a AVLTreeNode<K, V>>,
        get_item_func: fn(&'a AVLTreeNode<K, V>) -> R,
    ) -> Self {
        let next_node = root.as_ref().map(|root| {
            let iter = AvlTreeIterator {
                next_node: Some(root),
                get_item_func: get_key_value,
            };
            iter.leftmost_node(root)
        });

        Self {
            next_node,
            get_item_func,
        }
    }

    fn find_successor(&self, node: &'a AVLTreeNode<K, V>) -> Option<&'a AVLTreeNode<K, V>> {
        if let Some(right) = &node.right {
            return Some(self.leftmost_node(right));
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

    fn leftmost_node(&self, node: &'a AVLTreeNode<K, V>) -> &'a AVLTreeNode<K, V> {
        let mut current = node;

        while let Some(left) = &current.left {
            current = left;
        }

        current
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

impl<'a, K: Ord, V> IntoIterator for &'a AVLTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = AvlTreeKeyValueIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        AvlTreeKeyValueIterator::new(self.root.as_deref(), get_key_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::avl::AVLTree;

    #[test]
    fn test_get_insert() {
        let mut tree = AVLTree::new();

        assert_eq!(tree.get(&1), None);

        assert_eq!(tree.insert(1, 'a'), None);
        assert_eq!(tree.get(&1), Some(&'a'));
        assert_eq!(tree.get(&2), None);

        assert_eq!(tree.insert(2, 'b'), None);
        assert_eq!(tree.get(&1), Some(&'a'));
        assert_eq!(tree.get(&2), Some(&'b'));
        assert_eq!(tree.get(&3), None);

        assert_eq!(tree.insert(3, 'c'), None);
        assert_eq!(tree.get(&1), Some(&'a'));
        assert_eq!(tree.get(&2), Some(&'b'));
        assert_eq!(tree.get(&3), Some(&'c'));
        assert_eq!(tree.get(&4), None);

        assert_eq!(tree.insert(1, 'd'), Some('a'));
        assert_eq!(tree.insert(2, 'e'), Some('b'));
        assert_eq!(tree.insert(3, 'f'), Some('c'));

        assert_eq!(tree.get(&1), Some(&'d'));
        assert_eq!(tree.get(&2), Some(&'e'));
        assert_eq!(tree.get(&3), Some(&'f'));
        assert_eq!(tree.get(&4), None);
    }

    #[test]
    fn test_iterator() {
        let values = vec![(4, 'd'), (3, 'c'), (2, 'b'), (1, 'a')];

        let avl_tree = values.iter().cloned().collect::<AVLTree<_, _>>();
        let std_btree = values
            .iter()
            .cloned()
            .collect::<std::collections::BTreeMap<_, _>>();

        let avl_values = avl_tree.iter().collect::<Vec<_>>();
        let std_btree_values = std_btree.iter().collect::<Vec<_>>();

        assert_eq!(avl_values, std_btree_values);
    }
}
