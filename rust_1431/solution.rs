impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max_value: i32 = 0;
        let mut output = Vec::new();
        for i in candies.clone().into_iter() {
            if i > max_value {
                max_value = i;
            }
        }
        for i in candies.clone().into_iter() {
            if i+extra_candies >= max_value {
                output.push(true);
            } else {
                output.push(false);
            }
        }
        output
    }
}
