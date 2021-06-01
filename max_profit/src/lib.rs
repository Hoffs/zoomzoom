pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }

    let mut max_profit = 0;
    let mut min = prices[0];
    for x in prices {
        let profit = x - min;
        if profit > max_profit {
            max_profit = profit;
        }

        if x < min {
            min = x;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn it_works() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
