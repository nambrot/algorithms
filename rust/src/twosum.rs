use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub fn parse_file(filename: &str) -> Vec<isize> {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    s.lines().map(|str: &str| str.parse::<isize>().unwrap()).collect()
}

pub fn number_of_target_values_in(numbers: &Vec<isize>, start: isize, end: isize) -> isize {
  let mut table: HashSet<isize> = HashSet::new();
  for number in numbers {
    table.insert(number.clone());
  }

  let mut target_numbers: HashSet<isize> =  HashSet::new();

  for target_number in start..(end+1) {
    for number in numbers {
      if table.contains(&(target_number - number)) {
        target_numbers.insert(target_number);
        break;
      }
    }
  }

  target_numbers.len() as isize
}


