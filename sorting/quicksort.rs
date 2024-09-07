fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[0];
    let mut less = Vec::new();
    let mut greater = Vec::new();

    for &x in arr.iter().skip(1) {
        if x <= pivot {
            less.push(x);
        } else {
            greater.push(x);
        }
    }

    let mut less = less.into_iter().collect::<Vec<_>>();
    let mut greater = greater.into_iter().collect::<Vec<_>>();

    quicksort(&mut less);
    quicksort(&mut greater);

    arr.copy_from_slice(&[
        &less,
        &[pivot],
        &greater
    ].concat());
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr);

    quicksort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
