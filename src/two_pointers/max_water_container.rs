use std::cmp::min;

/// <summary>
/// You are given an integer array heights where heights[i] represents the height of the ith bar.
/// You may choose any two bars to form a container. Return the maximum amount of water a container can store.
/// </summary>
/// <example>
/// Input: height = [1,7,2,5,4,7,3,6]
/// Output: 36
/// </example>
/// <example>
/// Input: height = [2,2,2]
/// Output: 4
/// </example>
/// <constraints>
///    -- 2 <= height.length <= 1000
///    -- 0 <= height[i] <= 1000
/// </constraints>
#[allow(dead_code)]
pub fn max_area(heights: &[i32]) -> i32
{
    let mut l = 0usize;
    let mut r = heights.len().saturating_sub(1);
    //saturating_sub() subtracts a value respecting numeric bounds which avoids overflows, e.g. in case of length = 0
    let mut area = 0;

    while l < r
    {
        let a = (r - l) as i32 * min(heights[l], heights[r]);

        if a > area
        {
            area = a;
        }

        if heights[l] < heights[r]
        {
            l += 1;
        }
        else
        {
            r -= 1;
        }
    }

    area
}