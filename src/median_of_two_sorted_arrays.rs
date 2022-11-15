use std::collections::VecDeque;

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged_vec = merge_vectors(nums1, nums2);
    get_median(merged_vec)
}

fn merge_vectors(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut vec1 = VecDeque::from(vec1);
    let mut vec2 = VecDeque::from(vec2);

    while vec1.len() > 0 || vec2.len() > 0 {
        if vec1.len() == 0 {
            let popped = vec2.pop_front().unwrap();
            result.push(popped);
            continue;
        } else if vec2.len() == 0 {
            let popped = vec1.pop_front().unwrap();
            result.push(popped);
            continue;
        }

        let value1 = vec1.front().unwrap();
        let value2 = vec2.front().unwrap();

        if value1 < value2 {
            result.push(vec1.pop_front().unwrap());
        } else if value2 < value1 {
            result.push(vec2.pop_front().unwrap());
        } else {
            result.push(vec1.pop_front().unwrap());
            result.push(vec2.pop_front().unwrap());
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
