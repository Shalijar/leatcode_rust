impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_count = 0;
        let mut t_count = 0;
        while s_count < s.len() && t_count < t.len() {
            if s.as_bytes()[s_count] == t.as_bytes()[t_count]{
                s_count += 1;
                t_count += 1;
            } else {
                t_count += 1;
            }
        }
        if s_count == s.len() {
            return true;
        } else {
            return false;
        }
    }
}
