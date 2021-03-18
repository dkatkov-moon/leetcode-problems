pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut lo, mut hi) = (0, numbers.len() - 1);

    while lo < hi {
        let sum = numbers[lo] + numbers[hi];
        if target == sum {
            return vec![lo as i32 + 1, hi as i32 + 1];
        } else if sum < target {
            lo += 1;
        } else {
            hi -= 1;
        }
    }
    vec![-1, -1]
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3], "[2, 3, 4]");
    assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2], "[-1, 0]");
    assert_eq!(two_sum(vec![5, 25, 75], 100), vec![2, 3]);
}
