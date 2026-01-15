pub mod contains_duplicate;
pub mod valid_anagram;

#[cfg(test)]
mod tests {
    use crate::arrays_hashing::{contains_duplicate, valid_anagram};

    #[test]
    fn has_duplicate_test()
    {
        let nums1: [i32; 4] = [1, 2, 3, 4];
        assert!(!contains_duplicate::has_duplicate(&nums1));

        let nums2: [i32; 4] = [1, 2, 4, 4];
        assert!(contains_duplicate::has_duplicate(&nums2));

        let nums3: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 5, 10];
        assert!(contains_duplicate::has_duplicate(&nums3));

        let nums4: [i32; 20] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        assert!(!contains_duplicate::has_duplicate(&nums4));
    }

    #[test]
    fn valid_anagram_test()
    {
        let s1 = String::from("racecar");
        let t1 = String::from("carrace");
        assert!(valid_anagram::is_anagram(s1, t1));

        let s2 = String::from("jar");
        let t2 = String::from("jam");
        assert!(!valid_anagram::is_anagram(s2, t2));

        let s3 = String::from("anagram");
        let t3 = String::from("nagarram");
        assert!(!valid_anagram::is_anagram(s3, t3));

        let s4 = String::from("sadder");
        let t4 = String::from("dears");
        assert!(!valid_anagram::is_anagram(s4, t4));
    }
}