fn isPalindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let string1 = "madam";
    let string2 = "hello";

    println!("Is '{}' a palindrome? {}", string1, isPalindrome(string1));
    println!("Is '{}' a palindrome? {}", string2, isPalindrome(string2));
}