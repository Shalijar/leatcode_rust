impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let (mut output, mut temp, mut i) = (0, 0, 0);
        let mut left: Vec<i32> = Vec::new();
        for c in s.chars() {
            if i < k {
                if let 'a' | 'e' | 'i' | 'o' | 'u' = c.to_ascii_lowercase() {
                    left.push(i);
                    temp += 1;
                    output += 1;
                }
                i += 1;
                continue;
            }
            if left.len() > 0 && i-k == left[0] {
                left.remove(0);
                temp -= 1;
            }
            if let 'a' | 'e' | 'i' | 'o' | 'u' = c.to_ascii_lowercase() {
                temp += 1;
                left.push(i);
            }
            output = std::cmp::max(output, temp);
            i += 1;
        }
        output
    }
}
