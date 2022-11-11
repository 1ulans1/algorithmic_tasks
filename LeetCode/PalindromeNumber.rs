impl Solution {
    pub fn is_palindrome(palindrome: i32) -> bool {
        let mut help_number1 = palindrome;
        let mut help_number2 = 0;

        while palindrome > 0 {
            help_number2 += help_number1 % 10;
            help_number1 /= 10;
        }

        return help_number1 == palindrome
    }
}
