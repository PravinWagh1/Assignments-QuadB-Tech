fn MaxSubarraySum(arr: &[i32]) -> i32 {
    let mut maxSum = i32::MIN;
    let mut currentSum = 0;

    for &num in arr {
        currentSum = num.max(currentSum + num);
        maxSum = maxSum.max(currentSum);
    }

    maxSum
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", MaxSubarraySum(&arr));
}
