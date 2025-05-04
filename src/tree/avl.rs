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
    height: u32,
}

impl<K, V> AVLTreeNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            parent: std::ptr::null_mut(),
            height: 1,
        }
    }

    fn left_height(&self) -> u32 {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> u32 {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height());
    }

    fn balance_factor(&self) -> i8 {
        (self.left_height() as i64 - self.right_height() as i64) as i8
    }

    fn rotate_left(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        let Some(mut root) = node.take() else { return };

        let Some(mut right_child) = root.right.take() else {
            *node = Some(root);
            return;
        };

        let root_ptr = &mut *root;

        if let Some(left) = right_child.left.as_mut() {
            left.parent = root_ptr;
        }
        root.right = right_child.left.take();

        right_child.parent = root.parent;
        root.parent = &mut *right_child;

        root.update_height();

        right_child.left = Some(root);
        right_child.update_height();

        *node = Some(right_child);
    }

    fn big_rotate_left(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        if let Some(root) = node {
            Self::rotate_right(&mut root.right);
            Self::rotate_left(node);
        }
    }

    fn rotate_right(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        let Some(mut root) = node.take() else { return };

        let Some(mut left_child) = root.left.take() else {
            *node = Some(root);
            return;
        };

        let root_ptr = &mut *root;

        if let Some(right) = left_child.right.as_mut() {
            right.parent = root_ptr
        }
        root.left = left_child.right.take();

        left_child.parent = root.parent;
        root.parent = &mut *left_child;

        root.update_height();

        left_child.right = Some(root);
        left_child.update_height();

        *node = Some(left_child);
    }

    fn big_rotate_right(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        if let Some(root) = node {
            Self::rotate_left(&mut root.left);
            Self::rotate_right(node);
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

        None
    }

    fn get<Q: Borrow<K>>(&self, key: &Q) -> Option<&V> {
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

impl<K: Ord, V> AVLTree<K, V> {
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
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_get_insert_simple() {
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

    #[quickcheck]
    fn test_get_insert(values: Vec<(i32, char)>, keys: Vec<i32>) -> bool {
        let mut avl_tree = AVLTree::new();
        let mut std_btree = std::collections::BTreeMap::new();

        for (key, value) in values {
            if avl_tree.insert(key, value) != std_btree.insert(key, value) {
                return false;
            }
        }

        for key in keys {
            if avl_tree.get(&key) != std_btree.get(&key) {
                return false;
            }
        }

        true
    }

    #[quickcheck]
    fn test_iterator(values: Vec<(i32, char)>) -> bool {
        let avl_tree = values.iter().cloned().collect::<AVLTree<_, _>>();
        let std_btree = values
            .iter()
            .cloned()
            .collect::<std::collections::BTreeMap<_, _>>();

        let avl_values = avl_tree.iter().collect::<Vec<_>>();
        let std_btree_values = std_btree.iter().collect::<Vec<_>>();

        avl_values == std_btree_values
    }

    #[quickcheck]
    fn test_height(values: Vec<(i32, char)>) -> bool {
        let avl_tree = values.into_iter().collect::<AVLTree<_, _>>();

        avl_tree
            .nodes()
            .all(|node| node.height == 1 + node.left_height().max(node.right_height()))
    }

    #[quickcheck]
    fn test_balance_factor(values: Vec<(i32, char)>) -> bool {
        let avl_tree = values.into_iter().collect::<AVLTree<_, _>>();

        avl_tree
            .nodes()
            .all(|node| node.balance_factor().abs() <= 1)
    }

    #[test]
    fn playground() {
        let mut tree = AVLTree::new();

        tree.insert(0, '\0');
        println!("{}", tree.root.as_ref().unwrap().height);

        tree.insert(1, '\0');
        println!(
            "{}",
            tree.root
                .as_ref()
                .map(|node| node.right.as_ref().unwrap())
                .map(|node| node.height)
                .unwrap()
        );
        println!("{}", tree.root.as_ref().unwrap().height);
    }
}
