struct Solution;

impl Solution {
    /// brute force solution
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] + nums[j] == target && i != j {
                    indices.push(i as i32);
                    indices.push(j as i32);
                    return indices;
                }
            }
        }

        indices
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let indices = Solution::two_sum(nums, target);

    println!("{:?}", indices);
}
