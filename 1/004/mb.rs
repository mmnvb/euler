// At first I tried to find all the polindroms witihin the range
// 100*100 ... 999*999 and then try to evaluate whether they can be obtained by the factor of 3-digit numbers.
// I came with the solution that was equal to the correct answer, but I found my solution "lucky" and suitable for only those conditions. So, I rewrited
// boring, slow but the actually working code. 

fn is_polidrome(num: i32) -> bool{
    let str_num = num.to_string();

    if str_num == str_num.chars().rev().collect::<String>(){
        return true;
    }
    false
}


fn main(){
    let mut max = 0;

    // find all polidromes 
    for i in 100..1000{
        for r in 100..1000{
            let temp = i*r;
            if is_polidrome(temp) && temp>max{
                max = temp;
            }
        }
    }
    println!("{}", max);
}