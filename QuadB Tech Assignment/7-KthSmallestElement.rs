fn NthSmallestElement(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sortedArr = arr.to_vec();
        sortedArr.sort();
        Some(sortedArr[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];
    let k = 3;

    match NthSmallestElement(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is {}.", k, smallest),
        None => println!("Invalid input or k out of range."),
    }
}