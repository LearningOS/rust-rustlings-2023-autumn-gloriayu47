// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() -> Result<(),ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 解法：total_cost可能会抛出错误，然后会传播给main方法，所以加上Result<(),ParseIntError>
    // The unit (`()`) type is there because nothing is really needed in terms of positive results.
    let cost = total_cost(pretend_user_input)?;// 有错时抛出

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())// 无错时返回
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
