fn main() {
    println!("The sum of all the primes below two million: {}", sum_prime(2_000_000));
}
fn sum_prime(num: u64) -> u64{
    let mut sum = 0;
    for i in 2..num{
        if is_prime(i){
            sum = sum + i;
        }
    }
    sum
}
fn is_prime(n: u64) -> bool{
    if n == 2 || n == 3{
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0{
        return false;
    }
    let mut i = 5;
    
    while i * i <= n{
        if n % i == 0 || n % (i + 2) == 0{
            return false;
        }
        i += 6;
    }
    true
}
