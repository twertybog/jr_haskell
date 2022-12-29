fn main() {
    println!("The difference between the sum of the squares of the first {} 
    \rnatural numbers and the square of the sum: {}",
        100 ,sum_sq_diff(100));
}
fn sum_sq_diff(num: u64) -> u64{
    let sum_sq_n = (num * (num + 1) * (2 * num + 1)) / 6;

    let sum_nat = (num * (num + 1)) / 2;
    
    sum_sq_n.abs_diff(sum_nat.pow(2))
}