func twoSum(nums []int, target int) []int {
    numMap := make(map[int]int) // A mapping to store numbers and their indices
    for i, num := range nums {
        complement := target - num // Find the required number to reach the target
        if index, found := numMap[complement]; found {
            return []int{index, i} // Return indices of the complement and current number
        }
        numMap[num] = i // Store the number with its index
    }
    return nil // This line is never reached due to the problem guarantee
}