mod toolkit;
mod problem_1;
mod problem_2;

use crate::problem_1::sum;
use crate::problem_2::sum_loop;

fn main() {

    // Problem 1
    println!("Problem 1\n");
    toolkit::test(sum(3, 5), 8);
    toolkit::solution(sum(4274, 7657));

    // Problem 2
    // FIXME: Code abbey doesn't accept my solution, possibly because of incorrect copy-paste 
    println!("Problem 2\n");
    toolkit::test(sum_loop(vec![10, 20, 30, 40, 5, 6, 7, 8]), 126);
    toolkit::solution(sum_loop(vec![981, 1063, 200, 1182, 752, 666, 780, 1033, 438, 697, 862, 363, 64, 870, 543, 610, 1126, 370, 896, 602, 83, 624, 827, 1089, 487, 177, 396, 1299, 688, 67, 1259, 1236, 467, 1152, 92, 1094, 643, 519, 87, 1161, 254, 1286, 436, 1193]));
}
