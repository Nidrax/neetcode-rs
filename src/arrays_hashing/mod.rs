mod contains_duplicate;
mod valid_anagram;

fn has_duplicate_test()
{
    let nums1: [i32; 4] = [1, 2, 3, 4];
    let nums2: [i32; 4] = [1, 2, 4, 4];
    let nums3: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 5, 10];
    let nums4: [i32; 20] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    assert_eq!(contains_duplicate::has_duplicate(&nums1), false);
    assert_eq!(contains_duplicate::has_duplicate(&nums2), true);
    assert_eq!(contains_duplicate::has_duplicate(&nums3), true);
    assert_eq!(contains_duplicate::has_duplicate(&nums4), false);
}

fn valid_anagram_test()
{
    let s1 = String::from("racecar");
    let t1 = String::from("carrace");
    let s2 = String::from("jar");
    let t2 = String::from("jam");
    let s3 = String::from("anagram");
    let t3 = String::from("nagarram");
    let s4 = String::from("anagram");
    let t4 = String::from("nagaram");

    assert_eq!(valid_anagram::is_anagram(s1, t1), true);
    assert_eq!(valid_anagram::is_anagram(s2, t2), false);
    assert_eq!(valid_anagram::is_anagram(s3, t3), false);
    assert_eq!(valid_anagram::is_anagram(s4, t4), true);
}

pub fn arrays_hashing_tests()
{
    has_duplicate_test();
    valid_anagram_test();
}