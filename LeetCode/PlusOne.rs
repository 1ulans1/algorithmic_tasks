impl Solution {
    fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for index in (0..digits.len()).rev() {
            if digits[index] != 9 {
                digits[index] += 1;
                break;
            } else {
                digits[index] = 0;
            }
        };

        if digits[0] == 0 {
            let mut vec1 = vec![1];
            vec1.append(&mut digits);
            vec1
        } else {
            digits
        }
    }
}
