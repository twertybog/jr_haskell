fn main() {
    println!("The smallest multiple from 1 to {} is: {}", 20, smallest_mult(20));
}
fn smallest_mult(num: u64) -> u64{
    let mut counter: Vec<(u64, u32)> = Vec::new();
    for i in 2..num{
        let mut num = i;
        let mut buf: Vec<u64> = Vec::new();
        loop{
            if is_prime(num){
                buf.push(num);
                break;
            }
            let j = factor(&mut num);
            buf.push(j);
        }        
        is_ex(&mut counter, buf);
    }
    let mut result = 1;

    for i in counter{
        result = result * i.0.pow(i.1);
    }
    result
}

fn factor(num: &mut u64) -> u64{
    let mut i = 2;
    loop{
        if *num % i == 0{
            *num = match is_prime(i) {
                true => break,
                false => *num
            };
        }
        i+=1;
    }
    *num = *num / i;
    i
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

fn is_ex(counter: &mut Vec<(u64, u32)>, buf: Vec<u64>){
    for i in buf.clone(){
        let count = buf.iter()
            .fold(0, |acc, &n| if n == i { acc + 1 } else { acc });
        let mut found = false;
        for j in 0..counter.len(){
            if counter[j].0 == i{
                found = true;
                if count > counter[j].1{
                    counter[j].1 = count;                    
                }
            }
        }
        if !found{
            counter.push((i, count));
        }
    }
}