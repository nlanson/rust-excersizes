extern "C" {
    fn add(a: i32, b: i32, c: i32) -> i32;
}

fn main() {
    let a = unsafe { add(1, 2, 3)};

    println!("1+2+3={}", a);
}
