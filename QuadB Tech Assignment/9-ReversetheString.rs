fn reverseString(input: &str) -> String {
    let mut result = input.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = result.len() - 1;

    while left < right {
        result.swap(left, right);
        left += 1;
        right -= 1;
    }

    result.into_iter().collect()
}

fn main() {
    println!("Reversed string: {}", reverseString("Hello, world!"));
}