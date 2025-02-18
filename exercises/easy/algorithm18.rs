/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end],
    merge all overlapping intervals and return a list of non-overlapping intervals.

    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.

    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/


pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut merged = Vec::new();
    let mut current = intervals[0].clone();

    for interval in intervals.into_iter().skip(1) {
        if interval[0] <= current[1] {
            current[1] = current[1].max(interval[1]);
        } else {
            merged.push(current);
            current = interval;
        }
    }
    merged.push(current);
    return merged;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![0, 4]]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![4, 7], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![3, 7], vec![8, 10]]);
    }
}
