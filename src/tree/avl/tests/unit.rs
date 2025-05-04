use crate::tree::avl::{AVLTree, AVLTreeNode};
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
fn test_corner_cases() {
    let mut tree = AVLTree::new();

    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);

    tree.insert(1, 'a');
    assert!(!tree.is_empty());
    assert_eq!(tree.size(), 1);

    // TODO:
    // assert_eq!(tree.remove(&1), Some('a'));
    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);

    // Test with multiple identical values
    tree.insert(1, 'a');
    tree.insert(1, 'b');
    assert_eq!(tree.size(), 1);
    assert_eq!(tree.get(&1), Some(&'b'));
}

#[test]
fn test_iterator_empty_tree() {
    let tree: AVLTree<i32, char> = AVLTree::new();
    assert_eq!(tree.iter().next(), None);
}

#[test]
fn test_min_max() {
    let mut tree = AVLTree::new();

    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);

    tree.insert(3, 'c');
    tree.insert(1, 'a');
    tree.insert(5, 'e');
    tree.insert(2, 'b');
    tree.insert(4, 'd');

    assert_eq!(tree.min(), Some((&1, &'a')));
    assert_eq!(tree.max(), Some((&5, &'e')));
}

#[test]
fn test_specific_rotations() {
    // Test left rotation
    let mut tree = AVLTree::new();
    tree.insert(1, 'a');
    tree.insert(2, 'b');
    tree.insert(3, 'c');

    // After inserting 1, 2, 3 in this order, the tree should perform rotations
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test right rotation
    let mut tree = AVLTree::new();
    tree.insert(3, 'c');
    tree.insert(2, 'b');
    tree.insert(1, 'a');

    // After inserting 3, 2, 1 in this order, the tree should perform rotations
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test left-right rotation
    let mut tree = AVLTree::new();
    tree.insert(3, 'c');
    tree.insert(1, 'a');
    tree.insert(2, 'b');

    // After inserting 3, 1, 2 in this order, the tree should perform a double rotation
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test right-left rotation
    let mut tree = AVLTree::new();
    tree.insert(1, 'a');
    tree.insert(3, 'c');
    tree.insert(2, 'b');

    // After inserting 1, 3, 2 in this order, the tree should perform a double rotation
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);
}

#[test]
fn test_tree_structure() {
    let mut tree = AVLTree::new();
    tree.insert(5, 'e');
    tree.insert(3, 'c');
    tree.insert(7, 'g');
    tree.insert(2, 'b');
    tree.insert(4, 'd');
    tree.insert(6, 'f');
    tree.insert(8, 'h');

    // The tree should have a balanced structure now
    // Check root
    let root = tree.root.as_ref().unwrap();
    assert_eq!(root.key, 5);

    // Check the left subtree
    let left = root.left.as_ref().unwrap();
    assert_eq!(left.key, 3);
    assert_eq!(left.left.as_ref().unwrap().key, 2);
    assert_eq!(left.right.as_ref().unwrap().key, 4);

    // Check the right subtree
    let right = root.right.as_ref().unwrap();
    assert_eq!(right.key, 7);
    assert_eq!(right.left.as_ref().unwrap().key, 6);
    assert_eq!(right.right.as_ref().unwrap().key, 8);
}

#[quickcheck]
fn test_tree_invariants(values: Vec<(i32, char)>) -> bool {
    let tree = values.into_iter().collect::<AVLTree<_, _>>();

    // Check if the tree satisfies the BST property
    fn is_bst<K: Ord, V>(
        node: &Option<Box<AVLTreeNode<K, V>>>,
        min: Option<&K>,
        max: Option<&K>,
    ) -> bool {
        match node {
            None => true,
            Some(node) => {
                // Check the current node's key against bounds
                if let Some(min_key) = min {
                    if node.key <= *min_key {
                        return false;
                    }
                }

                if let Some(max_key) = max {
                    if node.key >= *max_key {
                        return false;
                    }
                }

                // Recursively check left and right subtrees
                is_bst(&node.left, min, Some(&node.key))
                    && is_bst(&node.right, Some(&node.key), max)
            }
        }
    }

    // Check if the tree is height-balanced
    fn is_balanced<K, V>(node: &Option<Box<AVLTreeNode<K, V>>>) -> bool {
        match node {
            None => true,
            Some(node) => {
                let balance_factor = node.balance_factor();
                balance_factor.abs() <= 1 && is_balanced(&node.left) && is_balanced(&node.right)
            }
        }
    }

    // Check if parent pointers are correct
    fn has_correct_parent_pointers<K, V>(
        node: &Option<Box<AVLTreeNode<K, V>>>,
        parent: *const AVLTreeNode<K, V>,
    ) -> bool {
        match node {
            None => true,
            Some(node) => {
                if node.parent as *const _ != parent {
                    return false;
                }

                has_correct_parent_pointers(&node.left, &**node)
                    && has_correct_parent_pointers(&node.right, &**node)
            }
        }
    }

    is_bst(&tree.root, None, None)
        && is_balanced(&tree.root)
        && has_correct_parent_pointers(&tree.root, std::ptr::null())
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
