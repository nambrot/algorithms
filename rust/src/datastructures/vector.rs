use std;

#[derive(Debug)]
pub struct Vector<T> {
  // Couldn't really figure out the easiest solution for dynamic (runtime) fixed size arrays
  data: Vec<Option<T>>,
  end_index: usize,
}

impl<T> Vector<T>
    where T: std::marker::Copy + std::cmp::PartialOrd + std::fmt::Debug
{
    pub fn new() -> Vector<T> {
        Vector { data: vec![None; 20], end_index: 0 }
    }

    pub fn push(&mut self, item: T) {
        if self.end_index >= self.data.len() {
          // Here is where you usually
          let mut new_data = vec![None; self.data.len() * 2];
          for i in 0..(self.data.len()) {
            new_data[i] = self.data[i];
          }
          self.data = new_data;
        }

        self.data[self.end_index] = Some(item);
        self.end_index += 1;
    }

    pub fn pop_from_end(&mut self) -> T {
      let new_end_index = self.end_index - 1;
      self.end_index = new_end_index;
      let item = self.get(self.end_index);
      self.unset(new_end_index);
      item
    }

    pub fn get(&self, index: usize) -> T {
      match self.data[index] {
        Some(item) => item,
        None => panic!("Tried to access item at {}, but no such thing", index)
      }
    }

    pub fn set(&mut self, index: usize, item: T) {
      self.data[index] = Some(item);
    }

    pub fn unset(&mut self, index: usize) {
      self.data[index] = None;
    }

    pub fn len(&self) -> usize {
      self.end_index
    }
}

#[test]
fn can_add() {
    let mut vector: Vector<usize> = Vector::new();
    vector.push(1);
    assert!(vector.get(0) == 1, "couldnt push and retrieve it correctly")
}

#[test]
fn can_set() {
    let mut vector: Vector<usize> = Vector::new();
    vector.push(1);
    vector.set(0, 2);
    assert!(vector.get(0) == 2, "couldnt push and retrieve it correctly")
}

#[test]
fn can_grow() {
    let mut vector: Vector<usize> = Vector::new();
    for i in 0..100 {
      vector.push(i);
      assert!(vector.get(i) == i, "didnt correctly push and grow");
    }
}

