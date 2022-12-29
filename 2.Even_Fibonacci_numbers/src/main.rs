fn main() {
    let num = 33;
    println!("The sum of even Fibonacci numbers: {}", even_fib_linear(num));
}

fn even_fib_linear(num: u64) -> u64{
    let mut first = 1;

    let mut second = 2;
    
    let mut sum = second;
    
    for i in 3..=num{
        if i%2 != 0{
            fibonacci(&mut first, &mut second);
        }
        else{
            fibonacci(&mut first, &mut second);
            sum = sum + second;
        }
    }

    sum
}

fn fibonacci(first: &mut u64, second: &mut u64){
    let buf: u64 = *second;

    *second = *second + *first;
    
    *first = buf;
}
