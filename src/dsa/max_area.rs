#[cfg(test)]
mod max_area {

    struct Solution;

    use std::cmp::{max, min};
    impl Solution {
        pub fn max_area(_height: Vec<i32>) -> i32 {
            struct SegTree {
                store: Vec<i32>,
                max_range: usize,
            }
            impl SegTree {
                pub fn new(max_range: usize) -> Self {
                    SegTree {
                        store: vec![i32::MAX; 4 * max_range],
                        max_range,
                    }
                }

                fn _update(
                    &mut self,
                    current_range_left: usize,
                    current_range_right: usize,
                    tree_index: usize,
                    index: usize,
                    value: i32,
                ) {
                    if current_range_left == current_range_right {
                        self.store[tree_index] = min(self.store[tree_index], value);
                        return;
                    }
                    let mid = (current_range_left + current_range_right) / 2;
                    if index >= current_range_left && index <= mid {
                        self._update(
                            current_range_left,
                            mid,
                            tree_index + tree_index + 1,
                            index,
                            value,
                        );
                    }
                    if index >= mid + 1 && index <= current_range_right {
                        self._update(
                            mid + 1,
                            current_range_right,
                            tree_index + tree_index + 2,
                            index,
                            value,
                        );
                    }

                    let left = self.store[tree_index + tree_index + 1];
                    let right = self.store[tree_index + tree_index + 2];

                    self.store[tree_index] = min(left, right);
                }

                pub fn update(&mut self, index: usize, value: i32) {
                    self._update(0, self.max_range, 0, index, value)
                }

                fn _get_min(
                    &self,
                    current_range_left: usize,
                    current_range_right: usize,
                    tree_index: usize,
                    range_left: usize,
                    range_right: usize,
                ) -> i32 {
                    if current_range_left >= range_left && current_range_right <= range_right {
                        return self.store[tree_index];
                    }

                    if current_range_left > range_right || current_range_right < range_left {
                        return i32::MAX;
                    }

                    let mid = (current_range_left + current_range_right) / 2;

                    return min(
                        self._get_min(
                            current_range_left,
                            mid,
                            tree_index + tree_index + 1,
                            range_left,
                            range_right,
                        ),
                        self._get_min(
                            mid + 1,
                            current_range_right,
                            tree_index + tree_index + 2,
                            range_left,
                            range_right,
                        ),
                    );
                }

                pub fn get_min(&self, range_left: usize, range_right: usize) -> i32 {
                    self._get_min(0, self.max_range, 0, range_left, range_right)
                }
            }

            let mut result = 0;
            let max_height = *_height.iter().max().unwrap();
            for rot in 0..2 {
                let mut seg_tree = SegTree::new(max_height as usize);
                let height = if rot == 1 {
                    _height.clone()
                } else {
                    _height.clone().into_iter().rev().collect::<Vec<i32>>()
                };

                height.into_iter().enumerate().for_each(|(index, h)| {
                    let farthest_index = seg_tree.get_min(h as usize, max_height as usize);
                    if farthest_index != i32::MAX {
                        result = max(result, ((index as i32) - farthest_index) * h);
                    }
                    seg_tree.update(h as usize, index as i32);
                });
            }

            result
        }
    }

    struct Solution2;
    impl Solution2 {
        fn max_area_helper(height: Vec<i32>) -> i32 {
            let mut prefix_max: Vec<i32> = vec![];
            height.iter().for_each(|h| {
                if prefix_max.is_empty() {
                    prefix_max.push(*h);
                } else {
                    prefix_max.push(max(*prefix_max.last().unwrap(), *h));
                }
            });

            height
                .into_iter()
                .enumerate()
                .skip(1)
                .fold(0, |acc, (index, h)| {
                    let mut low = 0 as i32;
                    let mut high = (index - 1) as i32;
                    while low <= high {
                        let g = (low + high) / 2;
                        if prefix_max[g as usize] >= h {
                            high = g - 1;
                        } else {
                            low = g + 1;
                        }
                    }
                    if low != (index as i32) {
                        max(acc, ((index as i32) - low) * h)
                    } else {
                        acc
                    }
                })
        }

        fn max_area(height: Vec<i32>) -> i32 {
            max(
                Solution2::max_area_helper(height.clone()),
                Solution2::max_area_helper(height.clone().into_iter().rev().collect::<Vec<i32>>()),
            )
        }
    }

    #[test]
    fn main() {
        // assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution2::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
