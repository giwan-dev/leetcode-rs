use std::{collections::HashMap, vec};

struct NumberTable {
    list: Vec<i32>,
    index_table: HashMap<i32, Vec<usize>>,
}

impl NumberTable {
    fn from(nums: Vec<i32>) -> Self {
        let mut table = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let mut indices = table.remove(num).unwrap_or(vec![]);
            indices.push(index);
            table.insert(num.clone(), indices);
        }

        Self {
            list: nums,
            index_table: table,
        }
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn pop(&mut self) -> (i32, usize) {
        let num = self.list.pop().unwrap();
        let (key, mut indices) = self.index_table.remove_entry(&num).unwrap();
        let index = indices.pop().unwrap();

        if indices.len() > 0 {
            self.index_table.insert(key, indices);
        }

        (num, index)
    }

    fn find_index(&mut self, num: i32) -> Option<usize> {
        if let Some(mut indices) = self.index_table.remove(&num) {
            let index = indices.pop().unwrap();

            Some(index)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numbers = NumberTable::from(nums);

    while numbers.len() > 0 {
        let (num1, index1): (i32, usize) = numbers.pop();

        if let Some(index2) = numbers.find_index(target - num1) {
            return vec![index1 as i32, index2 as i32];
        }
    }

    panic!("Fail to find indices.")
}

#[test]
fn basic() {
    fn sort_result(mut result: Vec<i32>) -> Vec<i32> {
        result.sort();
        result
    }

    assert_eq!(sort_result(two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    assert_eq!(sort_result(two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
    assert_eq!(sort_result(two_sum(vec![3, 3], 6)), vec![0, 1]);
    assert_eq!(sort_result(two_sum(vec![0, 4, 3, 0], 0)), vec![0, 3]);
}
