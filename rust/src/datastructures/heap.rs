use std;
extern crate rand;
use rand::Rng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Heap<T>
    where T: std::marker::Copy + std::cmp::PartialOrd + std::fmt::Debug
{
    pub data: Vec<Option<T>>,
}

impl<T> Heap<T>
    where T: std::marker::Copy + std::cmp::PartialOrd + std::fmt::Debug
{
    pub fn new() -> Heap<T> {
        Heap { data: vec![None; 20] }
    }

    pub fn push(&mut self, item: T) {
        let mut i = 0;

        while i < self.data.len() {
            match self.data[i] {
                Some(_) => i += 1,
                None => {
                    self.data[i] = Some(item);
                    self.balance_up(i);
                    return;
                }
            }
        }

        // Couldn't find a suitable spot -> grow!
        for _ in 1..self.data.len() {
            self.data.push(None);
        }
        println!("new length is {}", self.data.len());
        self.push(item);
    }

        pub fn peek(&self) -> &T {
        match self.data[0] {
            Some(ref item) => item,
            _ => panic!("peeked into empty heap"),
        }
    }

    pub fn pop(&mut self) -> T {
        match self.data[0] {
            Some(item) => {
                self.data[0] = None;
                self.move_last_item_to_root();
                item
            }
            _ => panic!("peeked into empty heap"),
        }
    }

    fn balance_up(&mut self, child_index: usize) {
        if child_index != 0 {
            let child = self.data[child_index].unwrap();
            let parent_index = self.parent_from(child_index);
            match self.data[parent_index] {
                Some(parent) if parent < child => {
                    self.swap(child_index, parent_index);
                    self.balance_up(parent_index);
                }
                _ => (),
            }
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let item = self.data[a];
        self.data[a] = self.data[b];
        self.data[b] = item;
    }

    fn parent_from(&self, source: usize) -> usize {
        (source - 1) / 2
    }



    fn balance_down(&mut self, from: usize) {
        let parent = self.data[from].unwrap();
        let first_child = if 2 * from + 1 > self.data.len() {
            None
        } else {
            self.data[2 * from + 1]
        };
        let second_child = if 2 * from + 2 > self.data.len() {
            None
        } else {
            self.data[2 * from + 2]
        };
        match (first_child, second_child) {
            (Some(child), Some(child2)) if parent < child && child < child2 => {
                self.swap(from, 2 * from + 2);
                self.balance_down(2 * from + 2);
            }
            (Some(child), Some(child2)) if parent < child && child > child2 => {
                self.swap(from, 2 * from + 1);
                self.balance_down(2 * from + 1);
            }
            (Some(child), _) if parent < child => {
                self.swap(from, 2 * from + 1);
                self.balance_down(2 * from + 1);
            }
            (_, Some(child)) if parent < child => {
                self.swap(from, 2 * from + 2);
                self.balance_down(2 * from + 2);
            }
            _ => (),
        }
    }

    fn move_last_item_to_root(&mut self) {
        let mut i = self.data.len() - 1;
        while i > 0 {
            match self.data[i] {
                None => i -= 1,
                Some(item) => {
                    self.data[0] = Some(item);
                    self.data[i] = None;
                    self.balance_down(0);
                    return;
                }
            }
        }
        assert!(self.data[0].is_none(), "We have data, but it wasnt moved");
    }


}

#[test]
fn can_add_and_peek_the_only_element() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(1);
    assert!(heap.peek() == &1, "Couldnt peek");
}

#[test]
fn can_add_and_peek_the_maximum_element() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(1);
    heap.push(0);
    assert!(heap.peek() == &1, "Couldnt peek the maximum");
}

#[test]
fn maintains_heap_property() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(0);
    heap.push(1);
    assert!(heap.peek() == &1, "Couldnt peek the maximum");
}

#[test]
fn pops_the_maximum() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(0);
    heap.push(1);
    assert!(heap.pop() == 1, "Couldnt pop the maximum");
}

#[test]
fn pops_the_maximum_again() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(0);
    heap.push(1);
    assert!(heap.pop() == 1, "Couldnt pop the maximum");
    assert!(heap.pop() == 0, "Couldnt pop the maximum again");
}

#[test]
fn heap_can_grow() {
    let mut heap: Heap<usize> = Heap::new();
    for i in 1..40 {
        heap.push(i);
    }
}

#[test]
fn balances_down_correctly() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(4);
    heap.push(1);
    heap.push(2);
    heap.push(0);
    heap.pop();
    assert!(heap.pop() == 2, "balanced to the right child");
    assert!(heap.pop() == 1, "balanced to the right child");
}

#[test]
fn benchmark_against_the_real_thing() {
    let mut real_heap: std::collections::BinaryHeap<usize> = std::collections::BinaryHeap::new();
    let mut heap: Heap<usize> = Heap::new();

    for _ in 1..100000 {
        let item = rand::thread_rng().gen_range(0, 100000000);
        real_heap.push(item);
        heap.push(item);
    }

    for _ in 1..100000 {
        let real_heap_item = real_heap.pop().unwrap();
        let heap_item = heap.pop();
        assert!(real_heap_item == heap_item,
                "implementation is wrong {} {}",
                real_heap_item,
                heap_item);
    }
}
