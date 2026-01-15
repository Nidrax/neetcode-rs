pub mod valid_palindrome;
mod max_water_container;

#[cfg(test)]
mod tests {
    use crate::two_pointers::{valid_palindrome, max_water_container};

    #[test]
    fn is_palindrome_test()
    {
        assert!(valid_palindrome::is_palindrome("Was it a car or a cat I saw?".to_string()));
        assert!(!valid_palindrome::is_palindrome("tab a cat".to_string()));
        assert!(valid_palindrome::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn max_water_test()
    {
        let a1 = [1,7,2,5,4,7,3,6];
        assert_eq!(max_water_container::max_area(&a1), 36);

        let a2 = [2,2,2];
        assert_eq!(max_water_container::max_area(&a2), 4);

        let a3 = [1, 1];
        assert_eq!(max_water_container::max_area(&a3), 1);

        let a4 = [0, 0, 0];
        assert_eq!(max_water_container::max_area(&a4), 0);

        let a5 = [1, 2, 3, 4, 5];
        assert_eq!(max_water_container::max_area(&a5), 6);

        let a6 = [5, 4, 3, 2, 1];
        assert_eq!(max_water_container::max_area(&a6), 6);

        let a7 = [3, 3, 3, 3];
        assert_eq!(max_water_container::max_area(&a7), 9);

        let a8 = [6, 1, 1, 1, 6];
        assert_eq!(max_water_container::max_area(&a8), 24);

        let a9 = [1, 100, 1, 100, 1];
        assert_eq!(max_water_container::max_area(&a9), 200);

        let a10 = [1000, 0, 1000];
        assert_eq!(max_water_container::max_area(&a10), 2000);

        let a11 = [0, 5];
        assert_eq!(max_water_container::max_area(&a11), 0);
    }
}