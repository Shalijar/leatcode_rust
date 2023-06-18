impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut output_string: String = "".to_string();
        let max_size: usize = cmp::max(word1.len(), word2.len());
        for i in 0..max_size {
            if i < word1.len() {
                output_string.push(word1.chars().nth(i).unwrap());
            }
            if i < word2.len() {
                output_string.push(word2.chars().nth(i).unwrap());
            }
        }
        return output_string;
    }
}