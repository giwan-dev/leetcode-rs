use std::cmp::Ordering;

#[allow(dead_code)]
fn search(nums: Vec<i32>, target: i32) -> bool {
    let first = nums.first().unwrap();

    match first.cmp(&target) {
        Ordering::Less => {
            let mut index = 1;

            loop {
                if index > (nums.len() - 1) {
                    return false;
                }

                if nums[index] >= target {
                    break;
                }

                index += 1;
            }
            nums[index] == target
        }
        Ordering::Equal => true,
        Ordering::Greater => {
            let mut index = nums.len() - 1;

            loop {
                if index == 0 {
                    return false;
                }

                if nums[index] <= target {
                    break;
                }

                index -= 1;
            }

            nums[index] == target
        }
    }
}

#[test]
fn basic() {
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    assert_eq!(search(vec![1], 0), false);
    assert_eq!(search(vec![1], 2), false);
    assert_eq!(search(vec![1, 1], 0), false);
}
