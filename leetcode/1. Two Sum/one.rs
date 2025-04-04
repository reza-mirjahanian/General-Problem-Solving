use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      let mut hash_table = HashMap::new();
      for(i, &num) in nums.iter().enumerate(){
          let c = target - num;
          if let Some(&inex) = hash_table.get(&c){
              return vec![inex as i32, i as i32]
          }
          hash_table.insert(num, i);

      }
      vec![]
    }
    
}

// Vec: The type in Rust’s standard library that represents a dynamically-sized, growable array.
// vec!: A macro used to create a Vec conveniently with initial elements, e.g., vec![1, 2, 3].
// So, Vec is the type, and vec! is a macro to instantiate it.


// iter gives access to the items in the vector without consuming or taking ownership.
// enumerate provides the index (i) along with the item, helping track the position in the collection.