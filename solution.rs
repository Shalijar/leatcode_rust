impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut left_product: Vec<i32> = Vec::with_capacity(size as usize);
        let mut right_product: Vec<i32> = Vec::with_capacity(size as usize);
        let mut output: Vec<i32> = Vec::with_capacity(size as usize);
        left_product.push(nums[0]);
        right_product.push(nums[size-1]);
        for i in 1..size {
            left_product.push(left_product[i-1] * nums[i]);
            right_product.push(right_product[i-1] * nums[size-i-1]);
        } 
        output.push(right_product[size-2]);
        for i in 1..(size-1) {
            output.push(left_product[i-1] * right_product[size-i-2]);
        }
        output.push(left_product[size-2]);
        return output;
    }
}
