use crate::tree::avl::AvlTree;
use quickcheck_macros::quickcheck;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[test]
fn test_complex_mixed_operations() {
    // Create a predictable sequence of operations
    let operations = [
        (true, 50, 'A'),  // Insert 50
        (true, 30, 'B'),  // Insert 30
        (true, 70, 'C'),  // Insert 70
        (true, 20, 'D'),  // Insert 20
        (true, 40, 'E'),  // Insert 40
        (true, 60, 'F'),  // Insert 60
        (true, 80, 'G'),  // Insert 80
        (false, 20, '_'), // Remove 20
        (true, 10, 'H'),  // Insert 10
        (true, 25, 'I'),  // Insert 25
        (false, 30, '_'), // Remove 30
        (true, 65, 'J'),  // Insert 65
        (false, 50, '_'), // Remove 50 (root)
        (true, 55, 'K'),  // Insert 55
        (true, 5, 'L'),   // Insert 5
        (false, 40, '_'), // Remove 40
        (true, 35, 'M'),  // Insert 35
        (false, 80, '_'), // Remove 80
        (true, 90, 'N'),  // Insert 90
        (false, 5, '_'),  // Remove 5
        (false, 65, '_'), // Remove 65
        (true, 50, 'O'),  // Insert 50 again
    ];

    // Keep track of expected state with a BTreeMap
    let mut expected = BTreeMap::new();
    let mut tree = AvlTree::new();

    for (idx, &(is_insert, key, value)) in operations.iter().enumerate() {
        if is_insert {
            let old_value = tree.insert(key, value);
            let expected_old_value = expected.insert(key, value);
            assert_eq!(
                old_value, expected_old_value,
                "Operation {}: Insert mismatch",
                idx
            );
        } else {
            let removed_value = tree.remove(&key);
            let expected_removed_value = expected.remove(&key);
            assert_eq!(
                removed_value, expected_removed_value,
                "Operation {}: Remove mismatch",
                idx
            );
        }

        // Verify tree state after each operation
        assert_eq!(
            tree.size(),
            expected.len(),
            "Size mismatch after operation {}",
            idx
        );

        // Verify the tree contains exactly the expected keys
        for (&k, &v) in expected.iter() {
            assert_eq!(
                tree.get(&k),
                Some(&v),
                "Key {} should be in tree after operation {}",
                k,
                idx
            );
        }

        // Verify the tree doesn't contain any unexpected keys (test 5 keys before and after each expected key)
        for &k in expected.keys() {
            for i in 1..=5 {
                if !expected.contains_key(&(k - i)) {
                    assert_eq!(
                        tree.get(&(k - i)),
                        None,
                        "Key {} should not be in tree after operation {}",
                        k - i,
                        idx
                    );
                }
                if !expected.contains_key(&(k + i)) {
                    assert_eq!(
                        tree.get(&(k + i)),
                        None,
                        "Key {} should not be in tree after operation {}",
                        k + i,
                        idx
                    );
                }
            }
        }

        // Verify AVL properties
        if !tree.is_empty() {
            assert!(
                tree.nodes().all(|node| node.balance_factor().abs() <= 1),
                "Tree not balanced after operation {}",
                idx
            );

            assert!(
                tree.nodes()
                    .all(|node| { node.height == 1 + node.left_height().max(node.right_height()) }),
                "Tree heights incorrect after operation {}",
                idx
            );
        }

        // Check iterator correctness
        let avl_keys: Vec<_> = tree.keys().collect();
        let expected_keys: Vec<_> = expected.keys().collect();
        assert_eq!(
            avl_keys, expected_keys,
            "Iterator produced wrong keys after operation {}",
            idx
        );

        let avl_values: Vec<_> = tree.values().collect();
        let expected_values: Vec<_> = expected.values().collect();
        assert_eq!(
            avl_values, expected_values,
            "Iterator produced wrong values after operation {}",
            idx
        );
    }
}

#[test]
fn test_large_tree_operations() {
    let mut tree = AvlTree::new();
    let mut reference = BTreeMap::new();

    // Insert 1000 items
    for i in 0..1000 {
        let key = i * 2; // Even numbers
        let value = (b'a' + (i % 26) as u8) as char;

        assert_eq!(tree.insert(key, value), reference.insert(key, value));

        // Periodically check tree invariants (avoid checking every iteration for performance)
        if i % 100 == 0 {
            assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
        }
    }

    assert_eq!(tree.size(), 1000);

    // Remove every other item (500 items)
    for i in (0..500).step_by(2) {
        let key = i * 2;
        assert_eq!(tree.remove(&key), reference.remove(&key));
    }

    assert_eq!(tree.size(), 750);

    // Add 250 odd-numbered items
    for i in 0..250 {
        let key = i * 2 + 1; // Odd numbers
        let value = (b'A' + (i % 26) as u8) as char;

        assert_eq!(tree.insert(key, value), reference.insert(key, value));
    }

    assert_eq!(tree.size(), 1000);

    // Verify all items are correctly stored
    for (key, value) in reference.iter() {
        assert_eq!(tree.get(key), Some(value));
    }

    // Check tree properties
    assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
    assert!(
        tree.nodes()
            .all(|node| { node.height == 1 + node.left_height().max(node.right_height()) })
    );

    // Check min/max
    assert_eq!(tree.min(), reference.iter().next());
    assert_eq!(tree.max(), reference.iter().next_back());
}

#[test]
fn test_extreme_imbalanced_insertion() {
    // Test inserting in ascending order
    let mut ascending_tree = AvlTree::new();
    for i in 0..100 {
        ascending_tree.insert(i, (b'a' + (i % 26) as u8) as char);

        // Verify balance after each insertion
        assert!(
            ascending_tree
                .nodes()
                .all(|node| node.balance_factor().abs() <= 1)
        );
    }

    // Test inserting in descending order
    let mut descending_tree = AvlTree::new();
    for i in (0..100).rev() {
        descending_tree.insert(i, (b'a' + (i % 26) as u8) as char);

        // Verify balance after each insertion
        assert!(
            descending_tree
                .nodes()
                .all(|node| node.balance_factor().abs() <= 1)
        );
    }

    // Both trees should be balanced and have correct heights
    assert!(
        ascending_tree
            .nodes()
            .all(|node| { node.height == 1 + node.left_height().max(node.right_height()) })
    );

    assert!(
        descending_tree
            .nodes()
            .all(|node| { node.height == 1 + node.left_height().max(node.right_height()) })
    );

    // Both trees should have the same set of keys
    let ascending_keys: Vec<_> = ascending_tree.keys().collect();
    let descending_keys: Vec<_> = descending_tree.keys().collect();
    assert_eq!(ascending_keys, descending_keys);
}

#[test]
fn test_zigzag_insertion_removal() {
    // Create a specific insertion pattern that tests all 4 rotation types
    let mut tree = AvlTree::new();

    // Insert in a pattern that causes zigzag insertions
    let insertions = [
        (50, 'a'),
        (25, 'b'),
        (75, 'c'), // Balanced tree
        (12, 'd'),
        (37, 'e'), // Left zigzag
        (62, 'f'),
        (87, 'g'), // Right zigzag
        (6, 'h'),
        (18, 'i'), // Left-Left
        (31, 'j'),
        (43, 'k'), // Left-Right
        (56, 'l'),
        (68, 'm'), // Right-Left
        (81, 'n'),
        (93, 'o'), // Right-Right
    ];

    for &(key, value) in &insertions {
        tree.insert(key, value);

        // Verify tree is always balanced
        assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
    }

    // Now remove nodes in a specific pattern to test rotations during removal
    let removals = [
        31, 81, // Simple leaf removals
        37, 62, // Nodes with one child
        25, 75, // Nodes with two children
        50, // Remove root
    ];

    for &key in &removals {
        tree.remove(&key);

        // Verify tree remains balanced after each removal
        assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
    }

    // Verify final tree structure
    assert_eq!(tree.size(), insertions.len() - removals.len());
}

#[test]
fn test_random_operations() {
    for _ in 0..1000 {
        let mut rng = rand::rng();

        let mut tree = AvlTree::new();
        let mut reference = BTreeMap::new();

        // Perform 1000 random operations
        for _ in 0..1000 {
            let operation = rng.random_range(0..=2); // 0: insert, 1: remove, 2: get
            let key = rng.random_range(0..500); // Use a limited key range to ensure collisions

            match operation {
                0 => {
                    // Insert
                    let value = (b'a' + (rng.random_range(0..26) as u8)) as char;
                    assert_eq!(tree.insert(key, value), reference.insert(key, value));
                }
                1 => {
                    // Remove
                    assert_eq!(tree.remove(&key), reference.remove(&key));
                }
                2 => {
                    // Get
                    assert_eq!(tree.get(&key), reference.get(&key));
                }
                _ => unreachable!(),
            }

            // Periodically check tree invariants
            if rng.random_range(0..20) == 0 {
                assert_eq!(tree.size(), reference.len());
                assert!(tree.nodes().all(|node| node.balance_factor().abs() <= 1));
                assert!(tree.check_parent_references());
            }
        }

        // Final verification
        assert_eq!(tree.size(), reference.len());
        assert!(tree.check_parent_references());

        for (key, value) in reference.iter() {
            assert_eq!(tree.get(key), Some(value));
        }

        // Verify iterators
        let tree_items: Vec<_> = tree.iter().collect();
        let reference_items: Vec<_> = reference.iter().collect();
        assert_eq!(tree_items, reference_items);
    }
}

#[test]
fn test_duplicate_keys() {
    let mut tree = AvlTree::new();

    // Insert a key multiple times
    assert_eq!(tree.insert(10, 'a'), None);
    assert_eq!(tree.insert(10, 'b'), Some('a'));
    assert_eq!(tree.insert(10, 'c'), Some('b'));

    // Check that only the latest value is stored
    assert_eq!(tree.get(&10), Some(&'c'));
    assert_eq!(tree.size(), 1);

    // Check that removing the key works
    assert_eq!(tree.remove(&10), Some('c'));
    assert_eq!(tree.get(&10), None);
    assert_eq!(tree.size(), 0);
}

#[test]
fn test_min_max_edge_cases() {
    let mut tree = AvlTree::<i32, char>::new();

    // Empty tree
    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);

    // Single element
    tree.insert(10, 'a');
    assert_eq!(tree.min(), Some((&10, &'a')));
    assert_eq!(tree.max(), Some((&10, &'a')));

    // Multiple elements
    tree.insert(5, 'b');
    tree.insert(15, 'c');
    assert_eq!(tree.min(), Some((&5, &'b')));
    assert_eq!(tree.max(), Some((&15, &'c')));

    // Remove min and max
    tree.remove(&5);
    assert_eq!(tree.min(), Some((&10, &'a')));

    tree.remove(&15);
    assert_eq!(tree.max(), Some((&10, &'a')));

    // Back to empty
    tree.remove(&10);
    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);
}

#[test]
fn test_iterator_coverage() {
    let mut tree = AvlTree::new();

    // Test iterators on empty tree
    assert_eq!(tree.iter().next(), None);
    assert_eq!(tree.keys().next(), None);
    assert_eq!(tree.values().next(), None);

    // Insert elements
    for i in 0..10 {
        tree.insert(i, (b'a' + i as u8) as char);
    }

    // Test key iterator
    let keys: Vec<_> = tree.keys().cloned().collect();
    let expected_keys: Vec<_> = (0..10).collect();
    assert_eq!(keys, expected_keys);

    // Test value iterator
    let values: Vec<_> = tree.values().cloned().collect();
    let expected_values: Vec<_> = (0..10).map(|i| (b'a' + i as u8) as char).collect();
    assert_eq!(values, expected_values);

    // Test key-value iterator
    let pairs: Vec<(_, _)> = tree.iter().map(|(k, v)| (*k, *v)).collect();
    let expected_pairs: Vec<_> = (0..10).map(|i| (i, (b'a' + i as u8) as char)).collect();
    assert_eq!(pairs, expected_pairs);

    // Test iterator after removals
    tree.remove(&3);
    tree.remove(&7);

    let keys_after_removal: Vec<_> = tree.keys().cloned().collect();
    let expected_keys_after_removal: Vec<_> = (0..10).filter(|&i| i != 3 && i != 7).collect();
    assert_eq!(keys_after_removal, expected_keys_after_removal);
}

#[test]
fn test_parent_pointers_correctness() {
    let mut tree = AvlTree::new();

    // Create a tree with various rotations
    for i in [5, 3, 7, 2, 4, 6, 8, 1, 9] {
        tree.insert(i, (b'a' + i as u8) as char);
    }

    assert!(tree.check_parent_references());

    // Perform a series of removals
    for i in [1, 5, 9] {
        tree.remove(&i);
        assert!(tree.check_parent_references());
    }
}

#[quickcheck]
fn test_all_invariants_maintained(operations: Vec<(bool, i32, char)>) -> bool {
    let mut tree = AvlTree::new();

    // Track all inserted keys
    let mut inserted_keys = HashSet::new();

    for (is_insert, key, value) in operations {
        if is_insert {
            tree.insert(key, value);
            inserted_keys.insert(key);
        } else if inserted_keys.contains(&key) {
            tree.remove(&key);
            inserted_keys.remove(&key);
        }

        if !tree.is_empty() {
            // Check AVL balance
            if !tree.nodes().all(|node| node.balance_factor().abs() <= 1) {
                return false;
            }

            // Check height correctness
            if !tree
                .nodes()
                .all(|node| node.height == 1 + node.left_height().max(node.right_height()))
            {
                return false;
            }

            // Check BST property
            fn is_bst<K: Ord, V>(
                node: &Option<Box<crate::tree::avl::node::AVLTreeNode<K, V>>>,
                min: Option<&K>,
                max: Option<&K>,
            ) -> bool {
                match node {
                    None => true,
                    Some(node) => {
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

                        is_bst(&node.left, min, Some(&node.key))
                            && is_bst(&node.right, Some(&node.key), max)
                    }
                }
            }

            if !is_bst(&tree.root, None, None) {
                return false;
            }

            // Check parent pointers
            fn check_parent_pointers<K, V>(
                node: &Option<Box<crate::tree::avl::node::AVLTreeNode<K, V>>>,
                expected_parent: *mut crate::tree::avl::node::AVLTreeNode<K, V>,
            ) -> bool {
                match node {
                    None => true,
                    Some(node) => {
                        if node.parent != expected_parent {
                            return false;
                        }

                        check_parent_pointers(&node.left, &**node as *const _ as *mut _)
                            && check_parent_pointers(&node.right, &**node as *const _ as *mut _)
                    }
                }
            }

            if !check_parent_pointers(&tree.root, std::ptr::null_mut()) {
                return false;
            }
        }
    }

    true
}
