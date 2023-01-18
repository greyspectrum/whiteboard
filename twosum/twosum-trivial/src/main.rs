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

fn main() {
    let nums = vec![2, 7, 11, 15];

    let target: i32 = 9;

    let print_solution = two_sum(nums, target);

    println!("The solution is: {:?}", print_solution);
}
