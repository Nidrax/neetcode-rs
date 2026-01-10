pub mod valid_palindrome;

#[cfg(test)]
mod tests {
    use crate::two_pointers::valid_palindrome;

    #[test]
    fn is_palindrome_test() {
        assert!(valid_palindrome::is_palindrome("Was it a car or a cat I saw?".to_string()));
        assert!(!valid_palindrome::is_palindrome("tab a cat".to_string()));
        assert!(valid_palindrome::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }
}