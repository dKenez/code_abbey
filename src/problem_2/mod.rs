use crate::problem_1::sum;

pub fn sum_loop(list: Vec<i32>) -> i32 {
    let mut accumulator = 0;

    for i in &list {
        accumulator = sum(accumulator, *i)
    }

    return accumulator;
}