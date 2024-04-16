fn findMedian(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let midRight = len / 2;
        let midLeft = midRight - 1;
        (arr[midLeft] + arr[midRight]) as f64 / 2.0
    } else {
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [1, 2, 3, 4, 5, 6];

    println!("Median of {:?}: {}", array1, findMedian(&array1));
    println!("Median of {:?}: {}", array2, findMedian(&array2));
}