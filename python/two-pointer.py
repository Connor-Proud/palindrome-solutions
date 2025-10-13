def is_palindrome(word:str)->bool:
    word=word.lower()
    left_pointer=0
    right_pointer=len(word)-1
    while left_pointer <= right_pointer:
        if word[left_pointer] != word[right_pointer]:
            return False
        left_pointer+=1
        right_pointer-=1
    return True
