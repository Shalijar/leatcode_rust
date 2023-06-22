impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut output: String = String::new();
        let mut vowel_char: Vec<char> = Vec::new();
        let mut vowel_index: Vec<usize> = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if let 'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' = c {
                vowel_char.push(c);
                vowel_index.push(i);
            
            }
        }
        if vowel_char.len() == 0 {
            output = s.clone();
            return output;
        }
        let mut counter: usize = 0;
        for (i, c) in s.chars().enumerate() {
            if i == vowel_index[counter] {
                output.push(vowel_char[vowel_char.len()-counter-1]);
                if counter + 1 < vowel_index.len() {
                    counter += 1;
                }
            } else {
                output.push(c);
            }
        }
        return output;
    }
}

