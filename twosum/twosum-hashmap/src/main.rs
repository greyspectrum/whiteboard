use std::collections::HashMap;

//pub fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
//    let mut map = HashMap::new();
//
//    for (i, x) in nums.iter().enumerate() {
//        let df = target - x;
//        if map.contains_key(&df) {
//            return (*map.get(&df).unwrap(), i);
//        }
//        map.insert(x, i);
//    }
//    return (0, 0);
//}

//fn main() {
//    let nums = vec![4, 5, 6];
//
//    let target: i32 = 11;
//
//    let ret = two_sum(nums, target);
//    println!("{}, {}", ret.0, ret.1);
//}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    let mut solution: Vec<i32> = Vec::new();

    for (i, x) in nums.iter().enumerate() {
        let df = target - x;
        if map.contains_key(&df) {
            let sum_a = *map.get(&df).unwrap() as i32;
            let sum_b = i as i32;
            solution.push(sum_a);
            solution.push(sum_b);
        }
        map.insert(x, i);
    }

    return solution;
}

fn main() {
    let nums = vec![10, 5, 6];

    let target: i32 = 15;

    let ret = two_sum(nums, target);
    println!("{:?}", ret);
}
