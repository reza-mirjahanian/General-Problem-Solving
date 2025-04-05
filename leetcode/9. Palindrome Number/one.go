func isPalindrome(x int) bool {
	if x < 0 {
		 return false
	 }
	 copied := x
	 reversed := 0
	 
	 for  x >  0 {
		 reversed = reversed * 10 + x % 10
		 x /= 10
	 }
	 
	 
	 return copied == reversed
 }