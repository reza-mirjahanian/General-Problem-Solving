impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
      
         // Convert the integer to a string
    let s = x.to_string();

    // Reverse the string and compare it to the original
    s.chars().rev().collect::<String>() == s
        
    }
}