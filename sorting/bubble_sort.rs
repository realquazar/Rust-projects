fn bubble_sort(arr: &mut [i32]) {
  let n = arr.len();

  for i in 0..n-1 {
    for j in 0..n-i-1 {
      if arr[j] > arr[j+1] {
        arr.swap(j, j+1);
      }
    }
  }
}

fn main() {
  let mut arr = [99, 3, 5, 44, 91, 67, 70];
  println!("Original array {:?}", arr);

  bubble_sort(&mut arr);
  println!("Sorted array {:?}", arr);
}
