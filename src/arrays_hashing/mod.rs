mod contains_duplicate;

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

pub fn arrays_hashing_tests()
{
    has_duplicate_test();
}