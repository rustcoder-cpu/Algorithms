// Algorithm: Sliding Window / Two Pointers
//
// 1. Keep a window between `i` and `j`.
// 2. Move `j` forward to expand the window.
// 3. `count` stores how many times each letter appears in the window.
// 4. `max_count` stores the highest frequency of any single letter
//    in the window.
// 5. If:
//
//        window_length - max_count > k
//
//    then we would need more than `k` replacements to make the entire
//    window consist of the same character.
//
// 6. If the window is invalid, move `i` forward to shrink the window.
// 7. If the window is valid, update `max_len` with the largest window found.
// 8. Continue until `j` reaches the end of the string.
//
// Time:  O(n)
// Space: O(1) because there are only 26 possible uppercase letters.

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let len = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut max_len = 0;
        let mut count = vec![0; 26];
        let mut max_count = 0;
        let chars: Vec<char> = s.chars().collect();

        while j < len && max_len < len - i {
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
        ("ABAB", 2),
        ("AABABBA", 1),
        ("AAAA", 2),
        ("ABCDE", 1),
        ("ABBB", 2),
        ("BAAAB", 2),
    ];

    for (s, k) in tests {
        let result = Solution::character_replacement(s.to_string(), k);

        println!("s = {s}, k = {k} -> result = {result}");
    }
}