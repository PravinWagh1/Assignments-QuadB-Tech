fn LongestCommonPrefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let smallestString = strings.iter().map(|s| s.len()).min().unwrap();

    let mut prefix = strings.iter().find_map(|s| {
        if s.len() == smallestString {
            Some(s.clone())
        } else {
            None
        }
    }).unwrap();

    for s in strings.iter().filter(|s| s != &&prefix) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
        if prefix.is_empty() {
            break;
        }
    }

    prefix
}

fn main() {
    let strings1 = vec![String::from("flower"), String::from("flow"), String::from("flight"),];
    let strings2 = vec![String::from("dog"), String::from("racecar"), String::from("car"),];

    println!("Longest common prefix of {:?}: {}", strings1, LongestCommonPrefix(&strings1));
    println!("Longest common prefix of {:?}: {}", strings2, LongestCommonPrefix(&strings2));
}
