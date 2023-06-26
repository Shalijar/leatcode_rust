impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut output: f64 = 0.0;
        let mut sum: f64 = 0.0;
        let mut left: usize = 0;
        for i in 0..nums.len() {
            if i < (k as usize) {
                sum += nums[i] as f64;
                output += nums[i] as f64;
                continue;
            } 
            sum += (-nums[left] + nums[i]) as f64;
            output = output.max(sum);
            left += 1;
        }
        output/(k as f64)
    }
}
