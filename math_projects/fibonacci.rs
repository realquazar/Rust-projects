// Prints the first 100 numbers of the fibonacci sequence

fn main() {
    let n = 100; 
    let mut a = 0;
    let mut b = 1;

    println!("Fibonacci sequence up to {}:", n);
    for _ in 0..n {
        println!("{}", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
}
