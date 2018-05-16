fn popcount(n: usize) -> usize {
    let mut count = 0;
    let mut _n = n;
    while _n != 0 {
        _n &= _n - 1;
        count += 1;
    }
    count
}

fn main() {
    println!("{}", popcount(0b11001101));
}
