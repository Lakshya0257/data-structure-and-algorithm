pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let n = garbage.len();
        let mut ans = 0;

        for i in 0..n - 1 {
            ans += 3 * travel[i];
        }

        for s in garbage.iter() {
            ans += s.len() as i32;
        }

        for i in (1..n).rev() {
            if !garbage[i].contains("G") {
                ans -= travel[i - 1];
            } else {
                break;
            }
        }

        for i in (1..n).rev() {
            if !garbage[i].contains("P") {
                ans -= travel[i - 1];
            } else {
                break;
            }
        }

        for i in (1..n).rev() {
            if !garbage[i].contains("M") {
                ans -= travel[i - 1];
            } else {
                break;
            }
        }

        ans
    }

fn main() {

}
