/// <summary>
/// Given a string s, return true if it is a palindrome, otherwise return false.
/// A palindrome is a string that reads the same forward and backward. It is also case-insensitive and ignores all non-alphanumeric characters.
/// Note: Alphanumeric characters consist of letters (A-Z, a-z) and numbers (0-9).
/// See: https://neetcode.io/problems/is-palindrome/question
/// </summary>
/// <example>
/// Input: s = "Was it a car or a cat I saw?"
/// Output: true
/// </example>
/// <example>
/// Input: s = "tab a cat"
/// Output: false
/// </example>
/// <constraints>
///    - 1 <= s.length <= 1000
///    - s is made up of only printable ASCII characters.
/// </constraints>
#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool
{
    let bytes = s.as_bytes();
    let mut l = 0usize;
    let mut r = bytes.len().saturating_sub(1);
    //saturating_sub() subtracts a value respecting numeric bounds which avoids overflows, e.g. in case of length = 0

    while l < r
    {
        //skip non-alphanumeric characters
        while l < r && !bytes[l].is_ascii_alphanumeric() {
            l += 1;
        }
        while l < r && !bytes[r].is_ascii_alphanumeric() {
            if r == 0 { break; }
            r -= 1;
        }
        if l >= r { break; }

        //compare left and right pointer until they cross
        if bytes[l].to_ascii_lowercase() != bytes[r].to_ascii_lowercase() {
            return false;
        }

        l += 1;
        if r == 0 { break; }
        r -= 1;
    }

    true
}
