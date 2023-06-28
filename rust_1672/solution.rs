impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut output = 0;
        for i in accounts {
            let mut sum = 0;
            for j in i {
                sum += j;
            }
            if sum > output {
                output = sum;
            }
        }
        output
    }
}
