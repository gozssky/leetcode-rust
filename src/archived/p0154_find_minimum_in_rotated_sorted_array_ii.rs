#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let (mut l, mut r) = (0, nums.len() - 1);
        while l + 1 < r {
            if nums[l] < nums[r] {
                return nums[l];
            }
            let m = (l + r) / 2;
            if nums[l] < nums[m] {
                l = m + 1;
            } else if nums[l] > nums[m] {
                r = m;
            } else {
                l += 1;
            }
        }
        nums[l].min(nums[r])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
