pub fn recursive_sum(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("{}", x);
    } else {
        return recursive_sum(x + 1, y - 1);
    }
}

pub fn main() {
    recursive_sum(2, 5);
}
