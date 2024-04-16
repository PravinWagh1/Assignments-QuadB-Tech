fn ShortestWord(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let str1 = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest) = ShortestWord(str1) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("No words found in the input string.");
    }
}
