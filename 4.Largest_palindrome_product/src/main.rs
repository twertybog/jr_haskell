fn main() {
    println!("The largest palindrome made from the product of two 3-digit numbers: {}", lag_palindrome_prod(3));
}

fn lag_palindrome_prod(digits: u32) -> u64{
    let max_val = 10_u64.pow(digits);

    let mut palindrome = 0;
    for i in (0..max_val).rev(){
        for j in (0..max_val).rev(){
            if is_palindrome(i*j){
                if i*j > palindrome{
                    palindrome = i*j;
                }
            }
        }
    }
    palindrome
}
fn is_palindrome(num: u64) -> bool{
    let mut buf = num;
    let mut rev_num = 0;
    while buf > 0{
        rev_num = rev_num * 10 + buf%10;
        buf/=10;
    }
    rev_num == num
}
