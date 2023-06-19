impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let output: &str = if str1.len() > str2.len() { &str2 } else { &str1 };
        Solution::reverse(&str1, &str2, output)
    }

    pub fn check(word1: &str, output: &str) -> bool {
        let mut word1_temp = word1;
        while word1_temp.len() >= output.len() {
            if &word1_temp[0..output.len()] == output {
                word1_temp = &word1_temp[output.len()..];
            } else {
                return false;
            }
        }
        word1_temp.is_empty()
    }

    pub fn reverse(str1: &str, str2: &str, output: &str) -> String {
        if output.is_empty() {
            String::new()
        }
        else if Solution::check(str1, output) && Solution::check(str2, output) {
            output.to_string()
        } else {
            Solution::reverse(
                &str1,
                &str2,
                &output[0..output.len() - 1],
            )
        }
    }
}