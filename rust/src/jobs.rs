use std;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    weight: isize,
    length: isize,
}

pub fn parse_file(filename: &str) -> Vec<Job> {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    let mut ret: Vec<Job> = vec![];
    for line in s.lines().skip(1) {
        let nums: Vec<isize> =
            line.split_whitespace().map(|num: &str| num.parse::<isize>().unwrap()).collect();
        ret.push(Job {
            weight: nums[0],
            length: nums[1],
        });
    }
    ret
}

// Difference scheduling
// fn key(job: &Job) -> isize {
//     -(job.weight/job.length)
// }

// Ratio scheduling
fn key(job: &Job) -> f32 {
    -(job.weight as f32/job.length as f32)
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        key(&self).partial_cmp(&key(&other)).unwrap_or(Less)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn schedule_jobs(jobs: &Vec<Job>) -> Vec<Job> {
  let mut arr = jobs.clone();
  arr.sort();
  arr
}

pub fn weighted_schedule_completion(jobs: &Vec<Job>) -> usize {
  let mut current_time = 0;
  let mut sum = 0;
  for job in jobs {
    current_time += job.length;
    sum += job.weight * current_time;
  }
  sum as usize
}
