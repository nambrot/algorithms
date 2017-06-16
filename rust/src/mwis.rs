use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

pub fn parse_file(filename: &str) -> Vec<u32> {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    s.lines()
        .map(|str: &str| str.parse::<u32>().unwrap())
        .skip(1)
        .collect()
}

pub fn maximum_weight_independent_set(numbers: &Vec<u32>) -> Vec<u32> {
  if numbers.len() < 2 {
    return numbers.clone();
  }

  let mut arr : Vec<u32> = vec![];
  arr.push(numbers[0]);
  arr.push(max(numbers[0], numbers[1]));

  let mut i = 2;
  while i < numbers.len() {
    let el = max(arr[i-2] + numbers[i], arr[i-1]);
    arr.push(el);
    i += 1;
  }

  assert!(numbers.len() == arr.len(), "Did not fully compute the array");

  // reconstruct
  i = numbers.len() - 1;
  let mut ret = vec![];
  while i >= 2 {
    let el = numbers[i];
    if arr[i] == arr[i-2] + el {
      ret.push(el);
      i -= 2;
    } else {
      i -= 1;
    }
  }

  if i == 1 {
    ret.push(max(numbers[1], numbers[0]));
  }

  if i == 0 {
    ret.push(numbers[0]);
  }

  ret.reverse();
  ret
}
