use std::cmp::min;

fn main() {
    println!("{}",minimum_coins(vec![1,10,10,12,1]));
}
// vec![26,18,6,12,49,7,45,45]

pub fn minimum_coins(prices: Vec<i32>) -> i32 {
    mini(&prices, 1, prices[0])
}

pub fn mini(prices: &Vec<i32>, index: usize, mut price: i32) -> i32 {

    if index > prices.len() {
        return price;
    }
    let mut temp_price = 0;
    for i in index..index+index+1 {
        if i > prices.len()-1 {
            return price;
        }

        // let x = (price+mini(prices, i + 1, price + prices[i-1])).min(price + mini(prices, i + 1, price + prices[i-1]));
        if i == index {
            temp_price += mini(prices, i+1,  price+prices[i]);
            println!("{}",temp_price);

        } else {
            temp_price=temp_price.min(mini(prices, i +1, price+prices[i]));
        };

    };

    temp_price
}