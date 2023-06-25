impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = nums.len() - 1;
        let mut i = 0; 
        while i < j {
            if nums[i] == 0 {
                let temp = nums.remove(i);
                nums.push(temp);
                j -= 1;
            } else {
                i += 1;
            }
        }
    }
}
