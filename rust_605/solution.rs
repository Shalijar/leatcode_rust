impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }

        flowerbed.insert(0, 0);
        flowerbed.push(0);
        for i in 1..flowerbed.len()-1 {
            if flowerbed[i-1] == 0 && flowerbed[i] == 0 && flowerbed[i+1] == 0 {
                n -= 1;
                if (n == 0) {
                    return true;
                }
                flowerbed[i] = 1;
            }
        }
        return false;
    }
}
