use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

#[derive(Debug)]
pub struct Entry {
    value: usize,
    weight: usize,
}

pub fn parse_file(filename: &str) -> (usize, Vec<Entry>) {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    let mut ret: Vec<Entry> = vec![];

    for line in s.lines().skip(1) {
        let nums: Vec<usize> = line.split_whitespace()
            .map(|num: &str| num.parse::<usize>().unwrap())
            .collect();
        ret.push(Entry {
                     value: nums[0],
                     weight: nums[1],
                 });
    }

    let first_line = s.lines().next().unwrap();
    let size = first_line
        .split_whitespace()
        .map(|num: &str| num.parse::<usize>().unwrap())
        .next()
        .unwrap();

    (size, ret)
}

pub fn sum(items: &Vec<Entry>) -> usize {
  items.iter().map(|e| e.value).fold(0, |acc, i| acc + i)
}

pub fn solve(items: &Vec<Entry>, capacity: usize) -> Vec<Entry> {
    // 2d array, indexed by item index and then capacity index
    let mut arr = vec![vec![0; capacity + 1]; items.len()];

    // populate the first row depending on if it the first element fits within capacity
    let mut j = 0;
    while j <= capacity {
        if items[0].weight <= j {
            arr[0][j] = items[0].value;
        } else {
            arr[0][j] = 0;
        }
        j += 1;
    }

    // "double for loop" to populate the computations
    let mut i = 1;
    while i < items.len() {
        j = 0;

        while j <= capacity {
            let item = &items[i];

            if item.weight > j {
                arr[i][j] = arr[i - 1][j];
            } else {
                arr[i][j] = max(arr[i - 1][j], arr[i - 1][j - item.weight] + item.value);
            }

            j += 1;
        }

        i += 1;
    }

    let mut ret: Vec<Entry> = vec![];
    // reconstruct which items we should pick
    i = items.len() - 1;
    j = capacity;
    let mut entry = &arr[i][j];
    while entry > &0 {
        let item = &items[i];

        if item.weight <= j {
            let possible_parent = arr[i - 1][j - item.weight];
            if entry == &(possible_parent + item.value) {
                // we picked this element
                ret.push(Entry {
                             weight: item.weight,
                             value: item.value,
                         });
                j -= item.weight;
            }
            // we did not pick the item
        }

        i -= 1;
        entry = &arr[i][j];

    }
    ret
}

