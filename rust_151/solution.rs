impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut output: String = String::new();
        for word in s.split_whitespace().rev() {
            output.push_str(word);
            output.push(' ');
        }
        output.truncate(output.len() - 1);
        return output;
    }
}
