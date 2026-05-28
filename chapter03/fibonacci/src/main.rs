fn main() {
    let N = 180;
    for i in 1..N {
        println!("Fibonacci {i} : {}", compute_fib(i));
    }
}

fn compute_fib(n: i128) -> i128 {
    if n < 1 { return -1 };
    let mut current: i128 = 1;
    let mut past: i128 = 0;
    for i in 1..n {
        let tmp = current + past;
        past = current;
        current = tmp;
    }
    current
}

