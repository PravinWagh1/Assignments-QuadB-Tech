fn firstOccurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]; 
    let target = 3; 

    match firstOccurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
        None => println!("{} is not found in the array.", target),
    }
}