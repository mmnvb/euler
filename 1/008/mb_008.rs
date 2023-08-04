// Algorithm of prime numbers: Sieve of Eratosthenes
fn is_prime(num: i32) -> bool{
    if num <=3{return true;}
    if num%2 == 0 || num % 3 == 0{
        return false;
    }

    let mut i=5;
    while i*i<=num{
        if num%i==0 || num%(i+2)==0{
            return false;
        }
        i += 6;
    }
    return true;
}


fn main(){
    let mut arr:Vec<i32> = Vec::new();

    // fill the arr
    for i in 2..2000000{
        if is_prime(i){
            arr.push(i);
        }
    }

    // find sum
    let mut summa:u64 = 0;
    for r in 0..arr.len(){
        summa += arr[r] as u64;
    }

    println!("Answer: {}", summa);
    println!("Should equal to: 142913828922");
}