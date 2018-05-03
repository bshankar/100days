use std::cmp::min;
struct Matrix(usize, usize);

fn mult(chain: &Vec<Matrix>) -> usize {
    let len = chain.len();
    let mut res = vec![vec![std::usize::MAX; len]; len];
    for i in 0..len {
        res[i][i] = 0;
    }

    for i in 1..len {
        for j in 0..len - i {
            for k in j..j + i {
                let cost = chain[j].0 * chain[k].1 * chain[j + i].1;
                res[j][j + i] = min(res[j][j + i], res[j][k] + res[k + 1][j + i] + cost);
            }
        }
    }
    res[0][len - 1]
}

fn main() {
    println!(
        "{}",
        mult(&vec![Matrix(10, 20), Matrix(20, 30), Matrix(30, 40)])
    );
}
