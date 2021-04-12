// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(1000000);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i64 {
    i64::from(num) * i64::from(num)
}
