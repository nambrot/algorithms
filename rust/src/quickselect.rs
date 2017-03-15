extern crate rand;
use rand::Rng;

pub fn select(x: &Vec<u32>, k: usize) -> usize {
    iterative_select(&mut x.clone(), k)
}

fn iterative_select(x: &mut Vec<u32>, k: usize) -> usize {
    let mut low = 0;
    let mut high = x.len();

    while low < high {
        let pivotpos = rand::thread_rng().gen_range(low, high);

        let pivot = x[pivotpos];

        x[pivotpos] = x[low];
        x[low] = pivot;

        let mut i = low + 1;
        let mut border = low;
        while i < high {
            if x[i] <= pivot {
                assert!(x[border] <= pivot,
                        "border prior to move was not maintained");
                border += 1;
                let val = x[i];
                x[i] = x[border];
                x[border] = val;
                assert!(x[border] <= pivot, "border post move was not maintained");
            }

            i += 1;
        }

        x[low] = x[border];
        x[border] = pivot;

        i = low;
        while i <= border {
            assert!(x[i] <= pivot, "border was not maintained in the lower part");
            i += 1;
        }

        while i < high {
            assert!(x[i] > pivot, "border was not maintained for upper part");
            i += 1;
        }

        if k - 1 == border {
            return pivot as usize
        } else if k - 1 > border {
            low = border + 1;
        } else {
            high = border;
        }
    }
    assert!(false, "exited while without returning");
    1
}

fn recursive_select(x: &mut Vec<u32>, k: usize, low: usize, high: usize) -> usize {
    let pivotpos = rand::thread_rng().gen_range(low, high);
    assert!(pivotpos < high, "pivot pos is invalid");

    let pivot = x[pivotpos];

    x[pivotpos] = x[low];
    x[low] = pivot;

    let mut i = low + 1;
    let mut border = low;
    while i < high {
        if x[i] <= pivot {
            assert!(x[border] <= pivot,
                    "border prior to move was not maintained");
            border += 1;
            let val = x[i];
            x[i] = x[border];
            x[border] = val;
            assert!(x[border] <= pivot, "border post move was not maintained");
        }

        i += 1;
    }

    x[low] = x[border];
    x[border] = pivot;

    i = low;
    while i <= border {
        assert!(x[i] <= pivot, "border was not maintained in the lower part");
        i += 1;
    }

    while i < high {
        assert!(x[i] > pivot, "border was not maintained for upper part");
        i += 1;
    }

    if k - 1 == border {
        pivot as usize
    } else if k - 1 > border {
        recursive_select(x, k, border + 1, high)
    } else {
        recursive_select(x, k, low, border)
    }
}
