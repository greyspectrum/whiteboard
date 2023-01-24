#![feature(test)]

extern crate test;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution: Vec<i32> = Vec::new();

    let mut found_solution: bool = false;

    let mut i: i32 = 0;

    while i < nums.len() as i32 && !found_solution {
        let mut j = i + 1;
        while j < nums.len() as i32 && !found_solution {
            if nums[i as usize] + nums[j as usize] == target {
                found_solution = true;
                solution.push(i);
                solution.push(j);
            }
            j += 1;
            i += 1;
        }
    }

    return solution;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        assert_eq!(vec![0, 1], two_sum(nums, target));
    }

    #[bench]
    fn bench_two_sum(b: &mut Bencher) {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        b.iter(|| two_sum(nums.clone(), target));
    }
}
