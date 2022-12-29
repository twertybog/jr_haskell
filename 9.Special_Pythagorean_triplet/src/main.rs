fn main() {
    let prod = match pythagorean_triplet(1000) {
        Some((a, b, c)) => a * b * c,
        None => 0
    };
    println!("The product abc one Pythagorean triplet for which a + b + c = 1000 is: {}", prod);
}

fn pythagorean_triplet(sum: u64) -> Option<(u64, u64, u64)>{
    for i in 1..sum/3{
        for j in 1..sum/2{
            let k = sum - i - j;
            if i * i + j * j == k * k
            {
                return Some((i, j, k));
            }
        }
    }
    None
}