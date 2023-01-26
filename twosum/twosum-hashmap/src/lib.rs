use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![10, 5, 6];

        let target: i32 = 15;

        assert_eq!(vec![0, 1], two_sum(nums,target));
    }
}
