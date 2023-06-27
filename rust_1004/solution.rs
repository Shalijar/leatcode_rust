impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut output, mut left, mut temp_k) = (0, 0, 0);
        for i in 0..nums.len() {
            if nums[i] == 0 {
                temp_k += 1;
            }
            if temp_k > k {
                if nums[left] == 0 {
                    temp_k -= 1;
                }
                left += 1;
            } 
            output = output.max(i - left + 1);
        }
        output as i32
    }
}
