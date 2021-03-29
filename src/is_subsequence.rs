// Problem 392
pub fn is_subsequence(mut s: String, t: String) -> bool {
    for ch in t.chars() {
        if s.is_empty() {
            return true;
        } else if ch == s.chars().nth(0).unwrap() {
            s.remove(0);
        }
    }

    if !s.is_empty() {
        return false;
    }
    return true;
}

fn main() {
    assert_eq!(is_subsequence("ace".to_string(), "abcde".to_string()), true);
    assert_eq!(
        is_subsequence("axc".to_string(), "ahbgdc".to_string()),
        false
    );

    assert_eq!(
        is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()),
        false
    );
}
