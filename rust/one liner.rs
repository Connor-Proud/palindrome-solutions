fn is_palindrome(word: &str) -> bool {
    let w = word.trim().to_lowercase();w == w.chars().rev().collect::<String>()
}
