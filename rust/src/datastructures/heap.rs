use std;
extern crate rand;
use rand::Rng;
use datastructures::vector::Vector;
use datastructures;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Heap<T>
    where T: std::marker::Copy + std::cmp::PartialOrd + std::fmt::Debug + std::cmp::Ord
{
    pub data: Vector<T>,
}

impl<T> Heap<T>
    where T: std::marker::Copy + std::cmp::PartialOrd + std::fmt::Debug + std::cmp::Ord
{
    pub fn new() -> Heap<T> {
        Heap { data: Vector::new() }
    }

    pub fn push(&mut self, item: T) {
        let len = self.data.len();
        self.data.push(item);
        self.balance_up(len);
    }

    pub fn peek(&self) -> T {
        self.data.get(0)
    }

    pub fn pop(&mut self) -> T {
        let item = self.data.get(0);
        let last_item = self.data.pop_from_end();
        self.data.set(0, last_item);
        self.balance_down(0);
        item
    }

    fn balance_up(&mut self, child_index: usize) {
        if child_index == 0 {
            return;
        }

        let child = self.data.get(child_index);
        let parent_index = self.parent_from(child_index);
        if self.data.get(parent_index) < child {
            self.swap(child_index, parent_index);
            self.balance_up(parent_index);
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let item = self.data.get(a);
        let other_item = self.data.get(b);
        self.data.set(a, other_item);
        self.data.set(b, item);
    }

    fn parent_from(&self, source: usize) -> usize {
        (source - 1) / 2
    }

    fn balance_down(&mut self, from: usize) {
        // Finding the largest element and balance based upon that
        struct BalanceCandidate<T> {
            index: usize,
            value: Result<T, datastructures::vector::ElementDoesNotExistError>,
        }
        struct FilteredBalanceCandidate<T> {
            index: usize,
            value: T,
        }

        let mut candidates: Vec<BalanceCandidate<T>> = vec![];
        candidates.push(BalanceCandidate {
            index: 0,
            value: self.data.get_as_result(from),
        });
        candidates.push(BalanceCandidate {
            index: 2 * from + 1,
            value: self.data.get_as_result(2 * from + 1),
        });
        candidates.push(BalanceCandidate {
            index: 2 * from + 2,
            value: self.data.get_as_result(2 * from + 2),
        });

        let mut filtered_candidates: Vec<FilteredBalanceCandidate<T>> = candidates.into_iter()
            .filter_map(|item| match item {
                BalanceCandidate { value: Ok(val), index } => Some(FilteredBalanceCandidate {
                    index: index,
                    value: val,
                }),
                _ => None,
            })
            .collect();

        filtered_candidates.sort_by(|a, b| a.value.cmp(&b.value).reverse());

        match filtered_candidates[0] {
            FilteredBalanceCandidate{ index, .. } if index > 0 => {
                self.swap(from, index);
                self.balance_down(index);
            }
            _ => (),
        }
    }
}

#[test]
fn can_add_and_peek_the_only_element() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(1);
    assert!(heap.peek() == 1, "Couldnt peek");
}

#[test]
fn can_add_and_peek_the_maximum_element() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(1);
    heap.push(0);
    assert!(heap.peek() == 1, "Couldnt peek the maximum");
}

#[test]
fn maintains_heap_property() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(0);
    heap.push(1);
    assert!(heap.peek() == 1, "Couldnt peek the maximum");
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
fn balances_down_deeply() {
    let mut heap: Heap<usize> = Heap::new();
    heap.push(41);
    heap.push(11);
    heap.push(12);
    heap.push(0);
    heap.push(7);
    heap.push(5);
    heap.push(6);
    heap.pop();
    assert!(heap.pop() == 12, "balanced to the right child");
    assert!(heap.pop() == 11, "balanced to the right child");
}

#[test]
fn benchmark_against_the_real_thing() {
    let mut real_heap: std::collections::BinaryHeap<usize> = std::collections::BinaryHeap::new();
    let mut heap: Heap<usize> = Heap::new();

    for _ in 1..10000 {
        let item = rand::thread_rng().gen_range(0, 100000);
        real_heap.push(item);
        heap.push(item);
    }

    for _ in 1..10000 {
        let real_heap_item = real_heap.pop().unwrap();
        let heap_item = heap.pop();
        assert!(real_heap_item == heap_item,
                "implementation is wrong {} {}",
                real_heap_item,
                heap_item);
    }
}
