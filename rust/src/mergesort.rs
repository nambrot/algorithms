use std;

pub fn sort(x: &Vec<u32>) -> Vec<u32> {
    iter_sort(x)
}

pub fn iter_sort(x: &Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = x.clone();
    let mut step = 1;

    while step < x.len() {
        let base = result.clone();
        result = vec![];
        let mut i = 0;

        loop {

          if i + step - 1 >= x.len() {
            let mut rest = x[i..(x.len())].to_vec();
            result.append(&mut rest);
            break;
          }

          let left = base[i..(i + step)].to_vec();
          let right_boundary = std::cmp::min(x.len(), i + step * 2);
          let right = base[(i + step)..right_boundary].to_vec();
          let mut merged = iter_merge(&left, &right);
          result.append(&mut merged);

          if right_boundary == x.len() {
            break;
          }
          i += step * 2;
        }
      step = step * 2;
    }
    result
}

pub fn iter_merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut acc: Vec<u32> = vec![];
    let mut lefti = 0;
    let mut righti = 0;

    loop {
        if lefti == left.len() && righti == right.len() {
            return acc;
        }

        if lefti < left.len() && righti < right.len() {
            if left[lefti] < right[righti] {
              acc.push(left[lefti]);
              lefti += 1;
              continue;
            } else {
              acc.push(right[righti]);
              righti += 1;
              continue;
            }
        }

        if lefti == left.len() {
          acc.push(right[righti]);
          righti += 1;
          continue;
        }

        if righti == right.len() {
          acc.push(left[lefti]);
          lefti += 1;
          continue;
        }
    }

}

// Problematic is that Rust does not have tail-call optimization
fn func_sort(x: &Vec<u32>) -> Vec<u32> {
    if x.len() == 1 {
        return x.clone();
    }
    let n = x.len() / 2;
    let x1 = func_sort(&x[0..n].to_vec());
    let x2 = func_sort(&x[n..x.len()].to_vec());
    merge(&x1, &x2, vec![])
}

fn merge(x: &Vec<u32>, y: &Vec<u32>, mut z: Vec<u32>) -> Vec<u32> {
    match (x.len(), y.len()) {
        (0, 0) => z,
        (0, _) => {
            merge(x, &y[1..y.len()].to_vec(), {
                z.append(&mut vec![y[0]]);
                z
            })
        }
        (_, 0) => {
            merge(&x[1..x.len()].to_vec(), y, {
                z.append(&mut vec![x[0]]);
                z
            })
        }
        (_, _) if x[0] < y[0] => {
            merge(&x[1..x.len()].to_vec(), y, {
                z.append(&mut vec![x[0]]);
                z
            })
        }
        _ => {
            merge(x, &y[1..y.len()].to_vec(), {
                z.append(&mut vec![y[0]]);
                z
            })
        }
    }
}
