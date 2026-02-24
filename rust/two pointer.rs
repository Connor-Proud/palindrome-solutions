fn is_palindrome(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
