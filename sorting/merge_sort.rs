fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(&mut arr, &left, &right);
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        }
        else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;    
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr);

    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
