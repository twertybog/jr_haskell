fn main() {
    println!("10001st prime is: {}", nt_prime(10001))
}
fn nt_prime(num: u64) -> u64{
    let mut counter = 0;
    let mut i = 2;
    while counter < num{
        if is_prime(i){
            counter += 1;
        }
        i+=1;
    }
    i-1
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