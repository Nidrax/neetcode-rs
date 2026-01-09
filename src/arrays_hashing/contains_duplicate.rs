use std::collections::HashMap;

/// <summary>
/// Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.
/// See: https://neetcode.io/problems/duplicate-integer/question
/// </summary>
/// <example>
/// Input: nums = [1, 2, 3, 3]
/// Output: true
/// </example>
/// <example>
/// Input: nums = [1, 2, 3, 4]
/// Output: false
/// </example>
pub fn has_duplicate(nums: &[i32]) -> bool
{
    let mut hashes = HashMap::new();

    for num in nums {
        if hashes.contains_key(num) { return true; }
        hashes.insert(num, num);
    }

    false
}
