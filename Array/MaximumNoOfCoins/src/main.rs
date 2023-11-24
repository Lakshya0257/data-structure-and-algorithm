fn main() {
    println!("{:?}",max_coins(vec![9,8,7,6,5,1,2,3,4]))
}

pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    let mut right = piles.len()-1;
    let mut left = 0;
    let mut coins = 0;

    while left<right {
        coins+=piles[right-1];
        right-=2;
        left+=1;
    }

    coins
}