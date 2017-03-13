use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub fn parse_file(filename: &str) -> Vec<u32> {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    s.lines().map(|str: &str| str.parse::<u32>().unwrap()).collect()
}

pub fn median(numbers: &Vec<u32>) -> u32 {
    let mut upper : BinaryHeap<ReverseU32> = BinaryHeap::new();
    let mut lower: BinaryHeap<u32> = BinaryHeap::new();

    for number in numbers {
        match upper.peek() {
            None => upper.push(ReverseU32 { el: number.clone() }),
            Some(&ReverseU32{el}) if number > &el => upper.push(ReverseU32 { el: number.clone() }),
            _ => lower.push(number.clone())
        }

        // Balance
        match (upper.len() as isize - lower.len() as isize) as isize {
            x if x > 1 => {
              let ReverseU32 {el} = upper.pop().unwrap();
              lower.push(el);
            }
            x if x < -1isize => {
              upper.push(ReverseU32 { el: lower.pop().unwrap() });
            }
            _ => {}
        }

        // Check invariances
        // assert!((upper.len() as i32- lower.len() as i32).abs() <= 1, "halfs are not balanced");
        // let median = median_from(&upper, &lower);
        // let slow_median = slow_median_from(&numbers, upper.len() + lower.len());
        // assert!(median == slow_median, "medians have not been maintained {} != {}", median, slow_median);
    }

    median_from(&upper, &lower)
}

fn median_from(upper: &BinaryHeap<ReverseU32>, lower: &BinaryHeap<u32>) -> u32 {
  if upper.len() > lower.len() {
    upper.peek().unwrap().el
  } else {
    lower.peek().unwrap().clone()
  }
}

// Method to calculate median slowly by sorting the array
fn slow_median_from(numbers: &Vec<u32>, number_of_items: usize) -> u32 {
  let mut x = numbers[0..number_of_items].to_vec();
  x.sort();
  if number_of_items % 2 == 0 {
    x[number_of_items / 2 - 1]
  } else {
    x[number_of_items / 2]
  }
}



#[derive(Clone, Debug, Eq, PartialEq)]
struct ReverseU32 {
  el: u32,
}

impl Ord for ReverseU32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.el.cmp(&other.el).reverse()
    }
}

impl PartialOrd for ReverseU32 {
    fn partial_cmp(&self, other: &ReverseU32) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

