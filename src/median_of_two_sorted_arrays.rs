use std::{cmp::Ordering, collections::VecDeque};

#[allow(dead_code)]
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    get_median(merge_vectors(nums1, nums2))
}

fn merge_vectors(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut vec1 = VecDeque::from(vec1);
    let mut vec2 = VecDeque::from(vec2);

    loop {
        match vec1.front() {
            Some(value1) => match vec2.front() {
                Some(value2) => match value1.cmp(value2) {
                    Ordering::Less => {
                        result.push(vec1.pop_front().unwrap());
                    }
                    Ordering::Equal => {
                        result.push(vec1.pop_front().unwrap());
                        result.push(vec2.pop_front().unwrap());
                    }
                    Ordering::Greater => {
                        result.push(vec2.pop_front().unwrap());
                    }
                },
                None => {
                    result.push(vec1.pop_front().unwrap());
                }
            },
            None => match vec2.front() {
                Some(_) => {
                    result.push(vec2.pop_front().unwrap());
                }
                None => {
                    break;
                }
            },
        }
    }

    result
}

fn get_median(nums: Vec<i32>) -> f64 {
    let length = nums.len();

    if length == 0 {
        return 0 as f64;
    }

    if length % 2 == 0 {
        let index1 = length / 2 - 1;
        let index2 = length / 2;

        (nums[index1] + nums[index2]) as f64 / 2.0
    } else {
        let index = length / 2;
        nums[index] as f64
    }
}

#[test]
fn basic() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
}
