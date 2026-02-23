function is_palindrome(word) {
    return word.toLowerCase().split('').reverse().join('') === word.toLowerCase();
}