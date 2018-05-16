fn eratosthenes(n: usize) -> usize {
    let mut sieve = vec![true; n / 2 + 1];
    for i in 0..sieve.len() {
        if sieve[i] {
            let prime = 2 * i + 3;
            let mut j = prime * prime / 2 - 1;
            while j < sieve.len() {
                sieve[j] = false;
                j += prime;
            }
        }
    }
    sieve.iter().filter(|n| **n).count()
}

fn main() {
    println!("{}", eratosthenes(1000000000));
}
