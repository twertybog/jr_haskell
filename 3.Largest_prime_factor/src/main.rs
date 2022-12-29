fn main() {
    let num: u64 = 600851475143;
    println!("The largest prime factor for {} is: {}", num, lag_prime_factor(num));
}

fn lag_prime_factor(num: u64) -> u64{
    let mut num = num;
    loop{
        if is_prime(num){
            break;
        }
        num = factor(num);
    }
    num
}

fn factor(mut num: u64) -> u64{
    let mut i = 2;
    loop{
        if num % i == 0{
            num = match is_prime(i) {
                true => break,
                false => num
            };
        }
        i+=1;
    }
    num / i
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