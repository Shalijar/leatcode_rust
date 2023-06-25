impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first_number = i32::MAX;
        let mut second_number = i32::MAX;
        for i in nums {
            if i <= first_number {
                first_number = i;
            } else if i <= second_number {
                second_number = i;
            } else {
                return true;
            }
        }
        return false;
    }
}

