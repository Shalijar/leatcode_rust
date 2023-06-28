impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::with_capacity(nums.len());
        output.push(nums[0]);
        for i in nums.iter().skip(1) {
            output.push(i + output[output.len()-1]);
        }
        output
    }
}
