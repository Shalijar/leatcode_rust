impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut s: String = String::new();
        s.push(chars[0]);
        let mut count: i32 = 1;
        for i in 1..chars.len() {
            if chars[i] == chars[i-1] {
                count += 1;
            } else {
                if count > 1 {
                    let temp = count.to_string();
                    for c in temp.chars() {
                    s.push(c);
                    }
                }   
                s.push(chars[i]);
                count = 1;
            }
        }
        if count > 1 {
            let temp = count.to_string();
            for c in temp.chars() {
                s.push(c);
            }
        } 
        chars.clear();
        for c in s.chars() {
            chars.push(c);
        }
        return chars.len() as i32;
    }
}
