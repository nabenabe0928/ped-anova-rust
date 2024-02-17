enum Side {
    Left,
    Right,
}

fn mean(xs: &[f64]) -> f64 {
    return xs.iter().sum::<f64>() / (xs.len() as f64);
}

fn stddev(xs: &[f64]) -> f64 {
    let n_samples = xs.len() as f64;
    let x_mean = xs.iter().sum::<f64>() / n_samples;
    let x_mean_squared = x_mean * x_mean;
    // let x_squred_mean = xs.iter().map(|&x| x * x).sum::<f64>() / n_samples;
    let x_squred_mean = xs.iter().map(|&x| x * x).sum::<f64>() / n_samples;
    return x_squred_mean - x_mean_squared;
}

fn linspace(start: f64, stop: f64, num: u32) -> Vec<f64> {
    let mut result = Vec::with_capacity(num as usize);
    if num == 0 {
        return result;
    } else if num == 1 {
        result.push(start);
        return result;
    }
    let slope = (stop - start) / ((num - 1) as f64);
    for i in 0..num {
        result.push(start + slope * i as f64);
    }
    return result;
}

fn searchsorted(a: &[f64], v: f64, side: Side) -> usize {
    let mut ok = (a.len() - 1) as i32;
    let mut ng = -1;
    // a[i-1] < v <= a[i]
    // a[i-1] <= v < a[i]
    while (ok - ng).abs() > 1 {
        let mid = ((ok + ng) / 2) as usize;
        let mut is_condition_ok: bool;
        match side {
            Side::Left => is_condition_ok = a[mid] >= v,
            Side::Right => is_condition_ok = a[mid] > v,
        };
        if is_condition_ok {
            ok = mid as i32;
        } else {
            ng = mid as i32;
        }
    }
    return ok as usize;
}

fn main() {
    let v = [1.0, 2.0];
    println!("{}", mean(&v));
    println!("{}", stddev(&v));
    let hoge = [0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0];
    println!("{}/2", searchsorted(&hoge, 1.0, Side::Left));
    println!("{}/5", searchsorted(&hoge, 1.0, Side::Right));
}
