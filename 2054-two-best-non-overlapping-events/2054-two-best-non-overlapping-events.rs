impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let l = events.len();
        for i in 0..l {
            events[i].push(0);

            let mut e = events[i].clone();
            e[3] = 1;
            events.push(e);
        }
        events.sort_by_key(|e| e[e[3] as usize]);

        let mut max_until = 0;
        let mut max_ret = 0;

        for e in events {
            if e[3] == 1 {
                max_until = max_until.max(e[2]);
            } else {
                max_ret = max_ret.max(max_until + e[2]);
            }
        }

        max_ret
    }
}