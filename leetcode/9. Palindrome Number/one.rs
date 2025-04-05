impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
      
      if x < 0 {
        return false
      }
      
      let mut copied = x;
      let mut reversed:i32 = 0;
      while copied > 0 {
        reversed = reversed * 10 + copied % 10;
        copied /= 10;
      }
      reversed == x
        
    }
}