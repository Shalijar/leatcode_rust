impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut i, mut j, mut output) = (0, nums.len()-1, 0);
        while i < j {
            if nums[i] > k  {
                break;
            }
            if nums[j] > k {
                j -= 1;
                continue;
            }           
            if nums[i] + nums[j] == k {
                output += 1;
                j -= 1;
                i += 1;
            } else if nums[i] + nums[j] < k {
                i += 1;
            } else {
                j -= 1;
            }
        }
        output
    }
}

