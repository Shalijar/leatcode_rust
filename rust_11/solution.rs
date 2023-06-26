impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut first = 1;
        let mut second = height.len();
        let mut output = 0;

        while first < second {
            output = std::cmp::max(output, (std::cmp::min(height[first-1],
                                    height[second-1]) * ((second - first) as i32)));
            if height[first-1] < height[second-1] {
                first += 1;
            } else {
                second -= 1;
            }
        }
           
        output
    }
}
