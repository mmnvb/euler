fn main() {
    let mut arr: Vec<i32> = vec![1, 2];
    let mut i = 0;
    let mut summa: i32 = 2;

    summa = loop{
        let temp = arr[i]+arr[i+1];
        arr.push(temp);
        
        if temp%2==0{
            summa = if temp<4000000 {summa+temp} else {break summa}
        }
        i += 1;
    };

    println!("{}", summa);
}
