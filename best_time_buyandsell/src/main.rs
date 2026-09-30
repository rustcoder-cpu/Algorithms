//                        A P P R O A C H
// Initialize variables buy with the first element of the prices array and profit as 0.
// Iterate through the prices starting from the second element.
// Update the buy variable if the current price is lower than the current buying price.
// Update the profit if the difference between the current price and the buying price is greater than the current profit.
// Return the final profit.

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
    println!("{}", max_profit([7, 1, 5, 3, 6, 4].to_vec()));
}