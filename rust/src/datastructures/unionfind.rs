use std;
use datastructures::vector::Vector;

#[derive(Debug)]
pub struct UnionFind {
    data: Vector<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = Vector::new();
        for i in 0..n {
            vec.push(i);
        }
        UnionFind { data: vec }
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

    // compress paths when applicable
    pub fn union(&mut self, a: usize, b: usize) {
        let root = self.find(a);
        let mut i = b;
        while i != root {
          let parent = self.data.get(i);
          self.data.set(i, root);
          i = parent;
        }
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
    let q = UnionFind::new(10);
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
    let mut q = UnionFind::new(10);
    assert!(!q.connected(1, 2));
    q.union(1, 2);
    assert!(q.connected(1, 2));
}

#[test]
fn can_union_deep() {
    let mut q = UnionFind::new(10);
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
