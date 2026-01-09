use std::collections::HashMap;

/// <summary>
/// Given two strings s and t, return true if the two strings are anagrams of each other, otherwise return false.
/// An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.
/// See: https://neetcode.io/problems/valid-anagram/question
/// </summary>
/// <example>
/// Input: s = "racecar", t = "carrace"
/// Output: true
/// </example>
/// <example>
/// Input: s = "jar", t = "jam"
/// Output: false
/// </example>
/// <constraints>
/// s and t consist of lowercase English letters.
/// </constraints>
#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool
{
    if s.len() != t.len() { return false; }
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        match char_count.get_mut(&ch) {
            Some(count) => {
                *count -= 1;
                if *count < 0 { return false; }
            },
            None => return false,
        }
    }

    for count in char_count.values() {
        if *count != 0 {
            return false;
        }
    }

    true
}