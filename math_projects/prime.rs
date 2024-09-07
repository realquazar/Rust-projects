// This program display the first 100 numbers of the prime numbers sequence

fn main() {
    let mut count = 0;
    let mut num = 2;

    loop {
        let mut is_prime = true;

        for i in 2..((num as f64).sqrt() as u32 + 1) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            count += 1;

            if count <= 100 {
                print!("{} ", num);
            } else {
                break;
            }
        }

        num += 1;
    }

    println!();
}
