use std;
use datastructures::vector::Vector;

pub struct Quickfind {
    data: Vector<usize>
}

impl Quickfind
{
    pub fn new(n: usize) -> Quickfind {
        let mut vec = Vector::new();
        for i in 0..n {
            vec.push(i);
        }
        Quickfind {
            data: vec
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn find(&self, item: usize) -> usize {
        self.data.get(item)
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root = self.find(a);
        let to_replace = self.find(b);
        let mut i = 0;
        let len = self.len();
        while i < len {
            if self.find(i) == to_replace {
                self.data.set(i, root);
            }
            i += 1;
        }
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[test]
fn can_construct() {
    let q = Quickfind::new(10);
    assert!(q.len() == 10, "Does not have the right size");
    assert!(q.find(1) == 1, "does not point to itself after init {}", q.find(1));
    assert!(!q.connected(1, 2), "incorrectly already connected after init");
    assert!(q.connected(1, 1), "is not connected to itself after init");
}

#[test]
fn can_union(){
    let mut q = Quickfind::new(10);
    assert!(!q.connected(1, 2));
    q.union(1, 2);
    assert!(q.connected(1, 2));
}
