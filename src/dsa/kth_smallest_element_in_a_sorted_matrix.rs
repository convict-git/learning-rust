#[cfg(test)]
mod problem {
    struct Solution;

    use std::collections::BinaryHeap;
    impl Solution {
        pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let n = matrix.len();
            let m = matrix.last().unwrap().len();
            let mut visited = vec![vec![false; m]; n];

            let mut pq = BinaryHeap::<(i32, (usize, usize))>::new();
            pq.push((-1 * matrix[0][0], (0, 0)));
            let mut count = k - 1;
            while count > 0 {
                let (_, (x, y)): (i32, (usize, usize)) = pq.pop().unwrap();
                visited[x][y] = true;

                if x + 1 < n && !visited[x + 1][y] {
                    pq.push((-1 * matrix[x + 1][y], (x + 1, y)));
                    visited[x + 1][y] = true;
                }
                if y + 1 < m && !visited[x][y + 1] {
                    pq.push((-1 * matrix[x][y + 1], (x, y + 1)));
                    visited[x][y + 1] = true;
                }
                count -= 1;
            }
            -1 * pq.peek().unwrap().0
        }
    }

    #[test]
    fn main() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 3, 5], vec![6, 7, 12], vec![11, 14, 14]], 6),
            11
        );
    }
}
