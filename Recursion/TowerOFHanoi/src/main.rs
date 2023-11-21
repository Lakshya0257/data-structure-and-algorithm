fn main() {
    let mut x = 10;
    toh(x, &mut String::from("World"))
}

fn toh(n: i32, x: &mut String) {
    if n == 0 {
        x.replacen()
        *x = String::from("Hello");
        println!("{}", n);
    } else {
        toh(n - 1, x);
        println!("{}", n);
    }
}
