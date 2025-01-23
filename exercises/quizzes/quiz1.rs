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

// 这是对以下部分的测验:
//-变量
//-函数
//-如果
//
// 玛丽正在买苹果。苹果的价格计算如下:
//-一个苹果要花2美元。
//-然而，如果玛丽买了40多个苹果，每个苹果的价格在
// 整个订单减少到只有1 rustbuck!

// TODO: 编写一个计算给定苹果订单价格的函数
// 购买的数量。

fn main() {
    // You can optionally experiment here.
}

fn calculate_price_of_apples(quantity: u32) -> u32 {
    if quantity > 40 {
        quantity
    } else {
        quantity * 2
    }
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
