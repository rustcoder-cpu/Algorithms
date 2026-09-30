pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        } else if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }
    profit
}

pub fn main() {
    println!({}, max_profit([7, 1, 5, 3, 6, 4]));
}