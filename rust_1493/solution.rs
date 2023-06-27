impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut left_container,
             mut right_container,
             mut output,) = (0, 0, 0);
        for i in 0..nums.len() {
            if nums[i] == 1 {
                left_container += 1;
                if i < nums.len()-1 {
                    continue;
                }
            }

            output = output.max(left_container + right_container);

            if left_container > 0 { // move data from left to right
                right_container = left_container;
                left_container = 0;

            }

            if i < nums.len() - 1 && nums[i] == 0 && nums[i+1] == 0 {
                // if there are two zeros, reset
                left_container = 0;
                right_container = 0;
            }
        }
        if output == nums.len() { // since all of the elements were 1, we have to remove one of them
            return (output-1) as i32;
        } else {
            return output as i32;
        }
    }
}
