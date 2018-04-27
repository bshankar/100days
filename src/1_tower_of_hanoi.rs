fn hanoi(height: usize, left: &String, right: &String, middle: &String) {
    if height > 0 {
        hanoi(height - 1, left, middle, right);
        println!("{} => {}", left, right);
        hanoi(height - 1, middle, right, left);
    }
}

fn main() {
    hanoi(
        3,
        &String::from("left"),
        &String::from("middle"),
        &String::from("right"),
    );
}
