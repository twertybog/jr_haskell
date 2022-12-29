fn main() {
    let num = 10000;
    println!("The sum of {} multiples is {}", num, mult_3_5(num)); 
}
fn mult_3_5(num: u32) -> u32{
    let mut sum = 0;
    sum = 3 * uint_sum((num - 1)/3);
    sum = sum + 5 * uint_sum((num - 1)/5);
    sum = sum - 15 * uint_sum((num - 1)/15);
    sum
}

fn uint_sum(n: u32) -> u32{
    (n * (n+1))/2
}
