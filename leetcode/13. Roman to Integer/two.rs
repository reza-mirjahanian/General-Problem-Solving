impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
          fn value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid Roman numeral character"),
        }
    }

    let mut total = 0;
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    if n == 0 {
        return 0;
    }

    for i in 0..n-1 {
        let current = value(chars[i]);
        let next_val = value(chars[i+1]);
        if current < next_val {
            total -= current;
        } else {
            total += current;
        }
    }

    total += value(chars[n-1]);
    total
    }
}