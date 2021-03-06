extern crate rand;
use rand::Rng;

pub fn sort(x: &mut Vec<u32>) {
    let end = x.len() - 1;
    sortt(x, 0, end);
}

fn sortt(x: &mut Vec<u32>, start: usize, end: usize) {
    if start == end {
        return;
    }
    // Get random pivot and put to the front
    let pivotpos = rand::thread_rng().gen_range(start, end + 1);
    let pivot = x[pivotpos];
    swap(x, start, pivotpos);

    let j = partition(x, start, end, pivot);

    if j != start {
        sortt(x, start, j - 1);
    }

    if j != end {
        sortt(x, j + 1, end);
    }

}

fn partition(x: &mut Vec<u32>, start: usize, end: usize, pivot: u32) -> usize {
    let mut i = start; // i-th element has been iterated over
    let mut j = start; // j-th element is <= pivot

    loop {
        i += 1;
        if x[i] <= pivot {
            j += 1;
            swap(x, i, j);
            assert!(is_ordered(x, start, j + 1, |&el| el <= pivot),
                    "Some elements are not smaller");
            assert!(is_ordered(x, j + 1, i, |&el| el > pivot),
                    "Some elements are not bigger");
        }
        if i == end {
            break;
        }

    }

    swap(x, start, j);
    assert!(is_ordered(x, start, j + 1, |&el| el <= pivot),
            "Some elements are not smaller");
    assert!(is_ordered(x, j + 1, i, |&el| el > pivot),
            "Some elements are not bigger");

            j
}

fn is_ordered<F>(x: &mut Vec<u32>, start: usize, end: usize, func: F) -> bool
    where F: Fn(&u32) -> bool
{
    if start <= end {
        x[start..end].to_vec().iter().all(func)
    } else {
        true
    }

}

fn swap(x: &mut Vec<u32>, a: usize, b: usize) {
    let tmp = x[a];
    x[a] = x[b];
    x[b] = tmp;
}
