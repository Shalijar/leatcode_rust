impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut sum, mut output) = (0, 0);
        for i in gain {
            sum += i;
            output = output.max(sum);
        }
        output
    }
}
