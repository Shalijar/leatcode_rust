impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut output: Vec<bool> = vec![false; 10];
        let mut n_1 = n;
        let mut n_2 = n*2;
        let mut n_3 = n*3;

        while n_1 > 0 || n_1 > 0 || n_1 > 0 {
            if n_1 > 0 {
                output[(n_1%10) as usize] = true;
                n_1 /= 10;
            }
            if n_2 > 0 {
                output[(n_2%10) as usize] = true;
                n_2 /= 10;
            }
            if n_3 > 0 {
                output[(n_3%10) as usize] = true;
                n_3 /= 10;
            }

            
        }
        if output[0] == true {
            return false;
        }
        for i in 1..10 {
            if output[i] == false {
                return false;
            }
        }
        true
    }
}
