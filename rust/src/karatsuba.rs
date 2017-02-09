use std::cmp::max;

pub fn mult(x: usize, y: usize) -> usize {
    func_mult(x, y)
}

fn func_mult(x: usize, y: usize) -> usize {
    match (x < 10, y < 10) {
        (true, true) => x * y,
        (_, _) => {
            let n = (max(x, y) as f32).log(10.0).ceil() as u32;
            let m = ((n as f32) / 2.0).floor() as u32;
            let x0 = x % 10usize.pow(m);
            let x1 = (x - x0) / 10usize.pow(m);
            let y0 = y % 10usize.pow(m);
            let y1 = (y - y0) / 10usize.pow(m);

            let z2 = func_mult(x1, y1);
            let z1 = func_mult(x1, y0) + func_mult(x0, y1);
            let z0 = func_mult(x0, y0);
            z2 * 10usize.pow(2 * m) + z1 * 10usize.pow(m) + z0
        }
    }
}
