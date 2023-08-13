use std::collections::HashMap;

/// LC 1. Two Sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices: HashMap<i32, i32> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;

        if indices.contains_key(&complement) {
            return vec![indices.get(&complement).unwrap().to_owned(), index as i32];
        } else {
            indices.insert(num.to_owned(), index as i32);
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::array::two_sum::two_sum;
    use test_case::test_case;

    #[test_case(vec![2, 7, 11, 15], 9, vec![0, 1];)]
    #[test_case(vec![3, 2, 4], 6, vec![1, 2];)]
    #[test_case(vec![3, 3], 6, vec![0, 1];)]
    fn test_examples(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(two_sum(nums, target), expected)
    }
}
