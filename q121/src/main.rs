fn main() {}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_p = i32::MAX;
    let mut profit = 0;
    let mut curr_profit = 0;

    for i in 0..prices.len() {
        if (prices[i] < min_p) {
            min_p = prices[i];
        }
        curr_profit = prices[i] - min_p;
        if (profit < curr_profit) {
            profit = curr_profit;
        }
    }
    return profit;
}
