pub fn sort(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() < 2 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left_half = sort(arr[..mid].to_vec());
    let right_half = sort(arr[mid..].to_vec());
    merge(left_half, right_half)
}

fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend(left[i..].into_iter());
    result.extend(right[j..].into_iter());
    result
}
