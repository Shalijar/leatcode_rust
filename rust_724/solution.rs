impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (mut left_sum, sum): (i32, i32) = (0, nums.iter().sum());
        for i in 0..nums.len() {
            if left_sum == sum - left_sum -nums[i] {
                return i as _;
            }
            left_sum += nums[i];
        }
        return -1;
    }
}
