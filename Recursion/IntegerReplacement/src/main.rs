fn main() {
    println!("Hello, world!");
}

    pub fn integer_replacement(mut n: i32) -> i32 {
        let mut n = n as u32;
        let mut steps = 0;
        while n > 1 {
            steps += 1;
            if n % 2 == 0 {
                n = n / 2;
                continue;
            }
            let plus = (n+1) / 2;
            if n != 3 && plus % 2 == 0 {
                n = n+1;
            } else {
                n = n-1;
            }
        }

        steps
    }

