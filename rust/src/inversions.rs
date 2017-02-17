use std;

pub fn count(x: &Vec<u32>) -> usize {
    let mut result: Vec<u32> = x.clone();
    let mut step = 1;
    let mut acc_count = 0;

    loop {
        if step > x.len() {
            break;
        }

        let base = result.clone();
        result = vec![];
        execute_step(&step, &base, &mut result, &mut acc_count);
        step = step * 2;
    }

    acc_count
}

fn execute_step(step: &usize, base: &Vec<u32>, acc: &mut Vec<u32>, acc_count: &mut usize) {
    let mut i = 0;
    loop {
        if i >= base.len() {
            break;
        }

        if i + step - 1 >= base.len() {
            let mut rest = base[i..(base.len())].to_vec();
            acc.append(&mut rest);
        } else {
            let left = base[i..(i + step)].to_vec();
            let right_boundary = std::cmp::min(base.len(), i + step * 2);
            let right = base[(i + step)..right_boundary].to_vec();

            iter_merge(&left, &right, acc, acc_count);
        }

        i += step * 2;
    }
}

pub fn iter_merge(left: &Vec<u32>, right: &Vec<u32>, acc: &mut Vec<u32>, acc_count: &mut usize) {
    let mut lefti = 0;
    let mut righti = 0;

    loop {
        if lefti == left.len() && righti == right.len() {
            break;
        } else if lefti < left.len() && righti < right.len() {
            if left[lefti] < right[righti] {
                acc.push(left[lefti]);
                lefti += 1;
            } else {
                acc.push(right[righti]);
                *acc_count = *acc_count + left.len() - lefti;
                righti += 1;
            }
        } else if lefti == left.len() {
            acc.push(right[righti]);
            righti += 1;
        } else if righti == right.len() {
            acc.push(left[lefti]);
            lefti += 1;
        }
    }
}
