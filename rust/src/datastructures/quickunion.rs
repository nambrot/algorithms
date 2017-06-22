use std;
use datastructures::vector::Vector;

#[derive(Debug)]
pub struct Quickunion {
    data: Vector<usize>,
}

impl Quickunion {
    pub fn new(n: usize) -> Quickunion {
        let mut vec = Vector::new();
        for i in 0..n {
            vec.push(i);
        }
        Quickunion { data: vec }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn find(&self, item: usize) -> usize {
        if self.is_root(item) {
            item
        } else {
            self.find(self.data.get(item))
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root = self.find(a);
        let old_root = self.find(b);
        self.data.set(old_root, root);
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn is_root(&self, a: usize) -> bool {
        self.data.get(a) == a
    }
}

#[test]
fn can_construct() {
    let q = Quickunion::new(10);
    assert!(q.len() == 10, "Does not have the right size");
    assert!(q.find(1) == 1,
            "does not point to itself after init {}",
            q.find(1));
    assert!(!q.connected(1, 2),
            "incorrectly already connected after init");
    assert!(q.connected(1, 1), "is not connected to itself after init");
}

#[test]
fn can_union() {
    let mut q = Quickunion::new(10);
    assert!(!q.connected(1, 2));
    q.union(1, 2);
    assert!(q.connected(1, 2));
}

#[test]
fn can_union_deep() {
    let mut q = Quickunion::new(10);
    assert!(!q.connected(1, 9));
    q.union(1, 2);
    q.union(8, 9);
    q.union(3, 2);
    q.union(6, 7);
    q.union(3, 4);
    q.union(6, 5);
    q.union(4, 5);
    q.union(7, 8);
    assert!(q.connected(1, 9));
}

#[test]
fn can_union_within_the_tree() {
    let mut q = Quickunion::new(10);
    assert!(!q.connected(1, 9));
    q.union(1, 3);
    q.union(3, 6);
    // no overwrite 6's root
    q.union(2, 6);
    assert!(q.connected(1, 6))
}
