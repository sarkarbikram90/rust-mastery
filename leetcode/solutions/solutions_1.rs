use std::collections::HashMap;

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut indices = HashMap::with_capacity(nums.len());

		for (index, value) in nums.into_iter().enumerate() {
			if let Some(&matching_index) = indices.get(&(target - value)) {
				return vec![matching_index as i32, index as i32];
			}

			indices.insert(value, index as i32);
		}

		unreachable!("the input is guaranteed to have exactly one solution")
	}
}
