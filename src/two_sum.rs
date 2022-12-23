use std::vec;

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index1, num1) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            if num1 + num2 == target {
                return vec![index1 as i32, index2 as i32];
            }
        }
    }

    panic!("Fail to find indices")
}

#[test]
fn basic() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
}
