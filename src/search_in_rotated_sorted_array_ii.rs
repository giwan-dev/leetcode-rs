#[allow(dead_code)]
fn search(nums: Vec<i32>, target: i32) -> bool {
    false
}

#[test]
fn basic() {
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}
