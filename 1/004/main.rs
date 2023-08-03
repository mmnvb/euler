fn is_palindrome(mut num: i32) -> bool {
    let mut digit: Vec<i32> = vec![0;10];
    let mut n: usize = 0;

    while num != 0 {
        digit[n] = num % 10;
        num = num / 10;
        n += 1;
    }

    for i in 0..n as usize {
        if digit[i] != digit[n-i-1] {
            return false;
        }
    }

    true
}

const MIN: i32 =100;
const MAX: i32 =1000;

fn main() {
    let mut result: i32;

    'out:
    for i in (MIN..MAX).rev() {
        for j in (MIN..MAX).rev() {
            result = i * j;
            if is_palindrome(result) {
                println!("{} x {} = {}", i, j, result);
                break 'out;
            }
        }
    }
}