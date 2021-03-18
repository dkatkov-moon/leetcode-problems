pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if target == (nums[i] + nums[j]) {
                drop(target);
                return vec![i as i32, j as i32];
            }
        }
        drop(nums[i]);
    }
    vec![0, 0]
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2], "[3, 2, 4]");
    assert_eq!(two_sum(vec![2, 5, 5, 10], 10), vec![1, 2], "[2, 5, 5, 10]");
}
