// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }
// Solution
// probably quite a needless use of const, but if contextually we know
// the actual cost of an apple doesn't change with the amount you buy,
// it's an offer that applies a discount to the cost of apples bought
// when you buy more than 40.

const APPLE_COST: i32 = 2;

fn calculate_price_of_apples(quant: i32) -> i32 {
    if quant <= 40 {
        APPLE_COST * quant
    }
    else {
        (APPLE_COST / 2) * quant
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
