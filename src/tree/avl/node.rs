pub struct AVLTreeNode<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Box<AVLTreeNode<K, V>>>,
    pub right: Option<Box<AVLTreeNode<K, V>>>,

    /// SAFETY: This pointer lives as long as the node itself.
    /// Used for iterator
    pub parent: *mut AVLTreeNode<K, V>,

    // TODO:
    // Так как высоты левых и правых поддеревьев в АВЛ-дереве отличаются максимум на 1
    // , то мы будем хранить не всю высоту дерева, а некоторое число, которое будет показывать, какое поддерево больше, или равны ли они, назовём фактор баланса. Таким образом в каждом узле будет храниться 1
    //  — если высота правого поддерева выше левого, 0
    //  — если высоты равны, и −1
    //  — если правое поддерево выше левого.
    // TODO: use stop_factor
    pub height: u32,
}

impl<K, V> AVLTreeNode<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            parent: std::ptr::null_mut(),
            height: 1,
        }
    }

    pub fn left_height(&self) -> u32 {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    pub fn right_height(&self) -> u32 {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    pub fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height());
    }

    pub fn balance_factor(&self) -> i8 {
        (self.left_height() as i64 - self.right_height() as i64) as i8
    }

    pub fn find_leftmost_node(&self) -> &AVLTreeNode<K, V> {
        let mut current = self;

        while let Some(left) = &current.left {
            current = left;
        }

        current
    }

    pub fn find_rightmost_node(&self) -> &AVLTreeNode<K, V> {
        let mut current = self;

        while let Some(right) = &current.right {
            current = right;
        }

        current
    }

    pub fn is_left_child(&self, other: &AVLTreeNode<K, V>) -> bool {
        self.left
            .as_ref()
            .is_some_and(|node| std::ptr::eq(&**node, other))
    }

    pub fn is_right_child(&self, other: &AVLTreeNode<K, V>) -> bool {
        self.right
            .as_ref()
            .is_some_and(|node| std::ptr::eq(&**node, other))
    }

    pub fn rotate_left(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        let Some(mut root) = node.take() else { return };

        let Some(mut right_child) = root.right.take() else {
            *node = Some(root);
            return;
        };

        if let Some(left) = right_child.left.as_mut() {
            left.parent = &mut *root;
        }
        root.right = right_child.left.take();

        right_child.parent = root.parent;
        root.parent = &mut *right_child;

        root.update_height();

        right_child.left = Some(root);
        right_child.update_height();

        *node = Some(right_child);
    }

    pub fn big_rotate_left(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        if let Some(root) = node {
            Self::rotate_right(&mut root.right);
            Self::rotate_left(node);
        }
    }

    pub fn rotate_right(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        let Some(mut root) = node.take() else { return };

        let Some(mut left_child) = root.left.take() else {
            *node = Some(root);
            return;
        };

        if let Some(right) = left_child.right.as_mut() {
            right.parent = &mut *root;
        }
        root.left = left_child.right.take();

        left_child.parent = root.parent;
        root.parent = &mut *left_child;

        root.update_height();

        left_child.right = Some(root);
        left_child.update_height();

        *node = Some(left_child);
    }

    pub fn big_rotate_right(node: &mut Option<Box<AVLTreeNode<K, V>>>) {
        if let Some(root) = node {
            Self::rotate_left(&mut root.left);
            Self::rotate_right(node);
        }
    }
}
