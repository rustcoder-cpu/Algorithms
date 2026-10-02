impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let len = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut max_len = 0;
        let mut count = vec![0; 26];
        let mut max_count = 0;
        let mut chars : Vec<char> = s.chars().collect(); 
        while (j < len && max_len < len - i ) {
            let j_char_index = (chars[j] as u32 - 'A' as u32) as usize;
            let i_char_index = (chars[i] as u32 - 'A' as u32) as usize;
            count[j_char_index] += 1;
            max_count = max_count.max(count[j_char_index]);
            let curr_len = j - i + 1;
            if curr_len - max_count > k as usize {
                count[i_char_index] -= 1;
                i += 1;
            } else {
                max_len = max_len.max(curr_len);
            }
            j += 1;
        }

        max_len as i32
    }
}

fn main() {
    let tests = [
        ("ABAB", 2, 4),
        ("AABABBA", 1, 4),
        ("AAAA", 2, 4),
        ("ABCDE", 1, 2),
        ("ABBB", 2, 4),
        ("BAAAB", 2, 5),
    ];

    for (s, k, expected) in tests {
        let result = Solution::character_replacement(s.to_string(), k);

        println!(
            "s = {:<10} k = {} -> result = {}, expected = {} {}",
            format!("\"{}\"", s),
            k,
            result,
            expected,
            if result == expected { "PASS" } else { "FAIL" }
        );
    }
}