extern "C" {
    fn add(a: i32, b: i32, c: i32) -> i32;
}

fn main() {
    let ret = unsafe { add(1, 2, 3) };

    println!("{}", ret);
}
