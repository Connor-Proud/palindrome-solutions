def is_palindrome(word:str)->bool:
    return True if word.lower()[::-1] == word else False

