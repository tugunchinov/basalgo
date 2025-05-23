use crate::tree::avl::{AVLTreeNode, AvlTree};
use quickcheck_macros::quickcheck;

#[test]
fn test_get_insert_simple() {
    let mut tree = AvlTree::new();

    assert_eq!(tree.get(&1), None);

    assert_eq!(tree.insert(1, 'a'), None);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&2), None);

    assert_eq!(tree.insert(1, 'b'), Some('a'));
    assert_eq!(tree.get(&1), Some(&'b'));

    assert_eq!(tree.insert(2, 'b'), None);
    assert_eq!(tree.get(&1), Some(&'b'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), None);

    assert_eq!(tree.insert(3, 'c'), None);
    assert_eq!(tree.get(&1), Some(&'b'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));
    assert_eq!(tree.get(&4), None);

    assert_eq!(tree.insert(1, 'd'), Some('b'));
    assert_eq!(tree.insert(2, 'e'), Some('b'));
    assert_eq!(tree.insert(3, 'f'), Some('c'));

    assert_eq!(tree.get(&1), Some(&'d'));
    assert_eq!(tree.get(&2), Some(&'e'));
    assert_eq!(tree.get(&3), Some(&'f'));
    assert_eq!(tree.get(&4), None);
}

#[quickcheck]
fn test_get_insert(values: Vec<(i32, char)>, keys: Vec<i32>) -> bool {
    let mut avl_tree = AvlTree::new();
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

#[test]
fn test_remove_simple() {
    let mut tree = AvlTree::new();

    assert_eq!(tree.remove(&1), None);

    assert_eq!(tree.insert(1, 'a'), None);
    assert_eq!(tree.insert(2, 'b'), None);
    assert_eq!(tree.insert(3, 'c'), None);

    assert!(tree.check_parent_references());

    assert_eq!(tree.remove(&1), Some('a'));

    assert_eq!(tree.get(&1), None);
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));

    assert!(tree.check_parent_references());

    assert_eq!(tree.remove(&2), Some('b'));

    assert!(tree.check_parent_references());

    assert_eq!(tree.get(&1), None);
    assert_eq!(tree.get(&2), None);
    assert_eq!(tree.get(&3), Some(&'c'));

    assert_eq!(tree.remove(&3), Some('c'));

    assert!(tree.check_parent_references());

    assert_eq!(tree.get(&1), None);
    assert_eq!(tree.get(&2), None);
    assert_eq!(tree.get(&3), None);

    assert_eq!(tree.remove(&1), None);
}

#[quickcheck]
fn test_iterator(values: Vec<(i32, char)>) -> bool {
    let avl_tree = values.iter().cloned().collect::<AvlTree<_, _>>();
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
    let avl_tree = values.into_iter().collect::<AvlTree<_, _>>();

    avl_tree
        .nodes()
        .all(|node| node.height == 1 + node.left_height().max(node.right_height()))
}

#[quickcheck]
fn test_balance_factor(values: Vec<(i32, char)>) -> bool {
    let avl_tree = values.into_iter().collect::<AvlTree<_, _>>();

    avl_tree
        .nodes()
        .all(|node| node.balance_factor().abs() <= 1)
}

#[test]
fn test_corner_cases() {
    let mut tree = AvlTree::new();

    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);

    tree.insert(1, 'a');
    assert!(!tree.is_empty());
    assert_eq!(tree.size(), 1);

    assert!(tree.check_parent_references());

    assert_eq!(tree.remove(&1), Some('a'));
    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);

    assert!(tree.check_parent_references());

    // Test with multiple identical values
    tree.insert(1, 'a');
    tree.insert(1, 'b');

    assert!(tree.check_parent_references());

    assert_eq!(tree.size(), 1);
    assert_eq!(tree.get(&1), Some(&'b'));
}

#[test]
fn test_iterator_empty_tree() {
    let tree: AvlTree<i32, char> = AvlTree::new();
    assert_eq!(tree.iter().next(), None);
}

#[test]
fn test_min_max() {
    let mut tree = AvlTree::new();

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
    let mut tree = AvlTree::new();
    tree.insert(1, 'a');
    tree.insert(2, 'b');
    tree.insert(3, 'c');

    // After inserting 1, 2, 3 in this order, the tree should perform rotations
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test right rotation
    let mut tree = AvlTree::new();
    tree.insert(3, 'c');
    tree.insert(2, 'b');
    tree.insert(1, 'a');

    // After inserting 3, 2, 1 in this order, the tree should perform rotations
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test left-right rotation
    let mut tree = AvlTree::new();
    tree.insert(3, 'c');
    tree.insert(1, 'a');
    tree.insert(2, 'b');

    // After inserting 3, 1, 2 in this order, the tree should perform a double rotation
    // to maintain balance. The root should end up being 2.
    let root_key = tree.root.as_ref().unwrap().key;
    assert_eq!(root_key, 2);

    // Test right-left rotation
    let mut tree = AvlTree::new();
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
    let mut tree = AvlTree::new();
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
    let tree = values.iter().cloned().collect::<AvlTree<_, _>>();

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
fn test_remove_cases() {
    let mut tree = AvlTree::new();

    // Build a specific tree structure for testing the three removal cases:
    //        5
    //       / \
    //      3   7
    //     / \ / \
    //    2  4 6  8

    tree.insert(5, 'e');
    tree.insert(3, 'c');
    tree.insert(7, 'g');
    tree.insert(2, 'b');
    tree.insert(4, 'd');
    tree.insert(6, 'f');
    tree.insert(8, 'h');

    assert!(tree.check_parent_references());

    // Case 1: Remove leaf node (2)
    assert_eq!(tree.remove(&2), Some('b'));
    assert_eq!(tree.get(&2), None);

    assert!(tree.check_parent_references());

    // Verify tree structure after leaf removal
    let root = tree.root.as_ref().unwrap();
    let left = root.left.as_ref().unwrap();
    assert_eq!(left.key, 3);
    assert!(left.left.is_none()); // Node 2 was removed
    assert_eq!(left.right.as_ref().unwrap().key, 4);

    // Case 2: Remove node with one child (3)
    assert_eq!(tree.remove(&3), Some('c'));
    assert_eq!(tree.get(&3), None);

    assert!(tree.check_parent_references());

    // Verify tree structure after one-child removal
    let root = tree.root.as_ref().unwrap();
    assert_eq!(root.key, 5);
    assert_eq!(root.left.as_ref().unwrap().key, 4); // Node 4 should have moved up

    // Case 3: Remove node with two children (7)
    assert_eq!(tree.remove(&7), Some('g'));
    assert_eq!(tree.get(&7), None);

    assert!(tree.check_parent_references());

    // Verify tree structure after two-children removal
    let root = tree.root.as_ref().unwrap();
    assert_eq!(root.key, 5);
    assert_eq!(root.right.as_ref().unwrap().key, 8); // Node 8 should have moved up
    assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().key, 6); // Node 6 should stay as left child
}

#[test]
fn test_remove_root() {
    let mut tree = AvlTree::new();

    // Simple case - tree with just the root
    tree.insert(1, 'a');
    assert_eq!(tree.remove(&1), Some('a'));
    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);

    assert!(tree.check_parent_references());

    // Root with two children
    tree.insert(2, 'b');
    tree.insert(1, 'a');
    tree.insert(3, 'c');

    assert!(tree.check_parent_references());

    assert_eq!(tree.remove(&2), Some('b'));
    assert_eq!(tree.get(&2), None);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&3), Some(&'c'));

    assert!(tree.check_parent_references());

    // Verify the new root is valid (either 1 or 3 depending on implementation)
    let root_key = tree.root.as_ref().unwrap().key;
    assert!(root_key == 1 || root_key == 3);
}

#[test]
fn test_remove_rebalancing() {
    let mut tree = AvlTree::new();

    // Create a tree that will need rebalancing after removal
    tree.insert(5, 'e');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 5);

    tree.insert(3, 'c');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 3);

    tree.insert(7, 'g');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 3);

    tree.insert(2, 'b');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 2);

    tree.insert(4, 'd');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 2);

    tree.insert(6, 'f');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 2);

    tree.insert(8, 'h');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 2);

    tree.insert(1, 'a');
    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 1);

    assert!(tree.check_parent_references());

    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 1);

    // Remove node 7 to trigger rebalancing
    assert_eq!(tree.remove(&7), Some('g'));

    assert!(tree.check_parent_references());

    assert_eq!(tree.root.as_ref().unwrap().find_leftmost_node().key, 1);

    // Verify the tree is still balanced
    assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));

    // Verify heights are correct
    assert!(
        tree.nodes()
            .all(|node| node.height == 1 + node.left_height().max(node.right_height()))
    );
}

#[test]
fn test_parent_pointers_after_removal() {
    let mut tree = AvlTree::new();

    // Build a test tree
    tree.insert(5, 'e');
    tree.insert(3, 'c');
    tree.insert(7, 'g');
    tree.insert(2, 'b');
    tree.insert(4, 'd');
    tree.insert(6, 'f');
    tree.insert(8, 'h');

    // Remove nodes that require different handling
    tree.remove(&2); // leaf node
    tree.remove(&7); // node with two children

    assert!(tree.check_parent_references());
}

#[test]
fn test_iterator_after_removal() {
    let mut tree = AvlTree::new();

    // Insert nodes
    tree.insert(3, 'c');
    tree.insert(1, 'a');
    tree.insert(5, 'e');
    tree.insert(2, 'b');
    tree.insert(4, 'd');

    // Remove a node in the middle
    tree.remove(&3);

    // Verify iterator traverses nodes in correct order
    let values: Vec<(&i32, &char)> = tree.iter().collect();
    assert_eq!(values.len(), 4);
    assert_eq!(values[0], (&1, &'a'));
    assert_eq!(values[1], (&2, &'b'));
    assert_eq!(values[2], (&4, &'d'));
    assert_eq!(values[3], (&5, &'e'));
}

#[test]
fn test_multiple_operations() {
    let mut tree = AvlTree::new();

    // Insert some nodes
    tree.insert(5, 'e');
    tree.insert(3, 'c');
    tree.insert(7, 'g');

    // Remove one
    assert_eq!(tree.remove(&3), Some('c'));

    // Insert more
    tree.insert(2, 'b');
    tree.insert(6, 'f');

    // Remove again
    assert_eq!(tree.remove(&5), Some('e'));

    // Verify final tree state
    assert_eq!(tree.size(), 3);
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&6), Some(&'f'));
    assert_eq!(tree.get(&7), Some(&'g'));

    // Verify balance
    assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
}

#[test]
fn test_remove_all() {
    // Test removing in ascending order
    let mut tree = AvlTree::new();
    for i in 1..=10 {
        tree.insert(i, (b'a' + (i - 1) as u8) as char);
    }

    for i in 1..=10 {
        assert_eq!(tree.remove(&i), Some((b'a' + (i - 1) as u8) as char));
        assert_eq!(tree.size(), 10 - i as usize);
    }
    assert!(tree.is_empty());

    // Test removing in descending order
    let mut tree = AvlTree::new();
    for i in 1..=10 {
        tree.insert(i, (b'a' + (i - 1) as u8) as char);
    }

    for i in (1..=10).rev() {
        assert_eq!(tree.remove(&i), Some((b'a' + (i - 1) as u8) as char));
        assert_eq!(tree.size(), i as usize - 1);
    }
    assert!(tree.is_empty());

    // Test removing in random order
    let mut tree = AvlTree::new();
    for i in 1..=10 {
        tree.insert(i, (b'a' + (i - 1) as u8) as char);
    }

    let order = [5, 2, 8, 1, 3, 7, 10, 4, 6, 9];
    for (idx, &i) in order.iter().enumerate() {
        assert_eq!(tree.remove(&i), Some((b'a' + (i - 1) as u8) as char));
        assert_eq!(tree.size(), 10 - idx - 1);
    }
    assert!(tree.is_empty());
}

#[quickcheck]
fn test_remove_operation(operations: Vec<(bool, i32, char)>) -> bool {
    let mut avl_tree = AvlTree::new();
    let mut std_btree = std::collections::BTreeMap::new();

    for (is_insert, key, value) in operations {
        if is_insert {
            // Insert operation
            if avl_tree.insert(key, value) != std_btree.insert(key, value) {
                return false;
            }
        } else {
            // Remove operation
            if avl_tree.remove(&key) != std_btree.remove(&key) {
                return false;
            }
        }

        // Verify both collections have the same content
        if avl_tree.iter().count() != std_btree.len() {
            return false;
        }

        for (k, v) in avl_tree.iter() {
            if std_btree.get(k) != Some(v) {
                return false;
            }
        }

        // Verify AVL tree properties
        if !avl_tree.is_empty() {
            if !avl_tree
                .nodes()
                .all(|node| node.balance_factor().abs() <= 1)
            {
                return false;
            }

            if !avl_tree
                .nodes()
                .all(|node| node.height == 1 + node.left_height().max(node.right_height()))
            {
                return false;
            }
        }
    }

    true
}

#[test]
fn test_complex_removal_sequence() {
    let mut tree = AvlTree::new();
    let operations = [
        (true, 50, 'A'),  // Insert 50
        (true, 25, 'B'),  // Insert 25
        (true, 75, 'C'),  // Insert 75
        (false, 25, '_'), // Remove 25
        (true, 10, 'D'),  // Insert 10
        (true, 60, 'E'),  // Insert 60
        (true, 80, 'F'),  // Insert 80
        (false, 50, '_'), // Remove 50 (root)
        (true, 5, 'G'),   // Insert 5
        (true, 15, 'H'),  // Insert 15
        (false, 80, '_'), // Remove 80
        (true, 90, 'I'),  // Insert 90
    ];

    let mut std_btree = std::collections::BTreeMap::new();

    for (is_insert, key, value) in operations.iter() {
        if *is_insert {
            tree.insert(*key, *value);
            std_btree.insert(*key, *value);
        } else {
            tree.remove(key);
            std_btree.remove(key);
        }

        // Verify tree state after each operation
        assert_eq!(tree.size(), std_btree.len());

        // Check AVL-specific invariants
        assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));

        // Check that the trees contain the same elements
        let avl_elements: Vec<_> = tree.iter().collect();
        let std_elements: Vec<_> = std_btree.iter().collect();
        assert_eq!(avl_elements, std_elements);
    }
}

#[test]
fn test_into_owned_iterator() {
    let mut tree = AvlTree::new();
    let data = vec![
        (4, "four"),
        (2, "two"),
        (6, "six"),
        (1, "one"),
        (3, "three"),
        (5, "five"),
        (7, "seven"),
    ];

    for (k, v) in &data {
        tree.insert(*k, *v);
    }

    let items: Vec<_> = tree.into_iter().collect();
    assert_eq!(
        items,
        vec![
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven")
        ]
    );
}

#[test]
fn test_consuming_iterator_vs_borrowing_iterator() {
    let mut tree1 = AvlTree::new();
    let mut tree2 = AvlTree::new();

    let data = vec![10, 5, 15, 3, 7, 12, 20, 1, 4, 6, 8, 11, 13, 18, 25];
    for &val in &data {
        tree1.insert(val, val.to_string());
        tree2.insert(val, val.to_string());
    }

    let borrowed: Vec<_> = tree1.iter().map(|(k, v)| (*k, v.clone())).collect();

    let consumed: Vec<_> = tree2.into_iter().collect();

    assert_eq!(borrowed, consumed);

    assert_eq!(tree1.size(), data.len());
}

#[test]
fn test_next_and_collect() {
    let mut tree = AvlTree::new();
    tree.insert(1, "one");
    tree.insert(2, "two");
    tree.insert(3, "three");

    let original_size = tree.size();
    assert_eq!(original_size, 3);

    let mut iter = tree.into_iter();

    assert_eq!(iter.next(), Some((1, "one")));

    let remaining: Vec<_> = iter.collect();
    assert_eq!(remaining, vec![(2, "two"), (3, "three")]);
}

#[test]
fn test_large_random_tree() {
    use std::collections::BTreeMap;

    let mut tree = AvlTree::new();
    let mut expected = BTreeMap::new();

    let values: [i32; 1000] = rand::random();
    for val in values {
        tree.insert(val, format!("value_{}", val));
        expected.insert(val, format!("value_{}", val));
    }

    let tree_items: Vec<_> = tree.into_iter().collect();
    let expected_items: Vec<_> = expected.into_iter().collect();

    assert_eq!(tree_items, expected_items);
}

#[test]
fn test_iterator_partial_consumption() {
    let mut tree = AvlTree::new();
    for i in 1..=10 {
        tree.insert(i, i.to_string());
    }

    let mut iter = tree.into_iter();

    assert_eq!(iter.next(), Some((1, "1".to_string())));
    assert_eq!(iter.next(), Some((2, "2".to_string())));
    assert_eq!(iter.next(), Some((3, "3".to_string())));

    drop(iter);
}

#[test]
fn playground() {
    let vals = vec![(7, 'a'), (5, 'b'), (10, 'c'), (6, 'd')];
    let tree = vals.into_iter().collect::<AvlTree<_, _>>();

    let vals = tree.iter().collect::<Vec<_>>();
    println!("{:?}", vals);
}
