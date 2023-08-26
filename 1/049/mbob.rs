// sieve of Eratosthenes
// check here https://www.youtube.com/watch?v=Lj_SzTGr-G4
fn is_prime(num: i32) -> bool{
    if num%2==0 || num%3==0{
        return false;
    }

    let mut i = 5;
    while i*i<num{
        if num%i==0 || num%(i+2)==0{
            return false;
        }
        i += 6;
    }
    return true;
}

trait Sorter {
    fn sort(&self) -> String;
}

impl Sorter for String{
    fn sort(&self) -> String{
        let mut temp:Vec<char> = self.chars().collect();
        temp.sort();
        temp.into_iter().collect()
    }
}


fn main(){
    use std::collections::HashMap;
    // vars
    let mut arr:Vec<i32> = Vec::new();
    let mut dict:HashMap<String, Vec<i32>> = HashMap::new();

    // step 1 - Find all primes from 1000 - 9999    
    for i in 1000..10000{
        if is_prime(i){arr.push(i)};
    }

    // step 2 - Sort every element. Sorted element : Vec<Elements>
    for num in arr.iter(){
        let temp_vec = dict.entry(num.to_string().sort()).or_insert(vec![0]);

        if temp_vec[0] == 0{
            temp_vec[0] = *num;
            continue;
        }
        temp_vec.push(*num);
    }

    // step 3 - Find the ariphmetic sequence. 
    // The tricky thing here is that actually there are 8 numbers that are prime and permutation of each other in the given example
    
    // govnokod here :)
    for i in dict.iter(){
        if i.1.len() >= 3{
            let mut remainings:HashMap<i32, Vec<i32>> = HashMap::new();

            for pointer in (0..i.1.len()).rev(){
                for sub in 0..pointer{
                    let key = i.1[pointer]-i.1[sub];
                    if key == 0 {continue;}

                    let temp_nums = remainings.entry(key).or_insert(vec![0]);
                    if temp_nums[0] == 0{
                        temp_nums[0] = i.1[pointer];
                        continue;
                    }
                    if &i.1[pointer] == temp_nums.last().unwrap(){
                        continue;
                    }
                    temp_nums.push(i.1[pointer]);
                    temp_nums.push(i.1[sub]);
                }
            }

            for last in remainings.iter(){
                if (last.1.len() == 3) && (last.1[0]-last.1[1] == last.1[1]-last.1[2]){
                    println!("FOUND {:?} and the ANSWER IS: {}{}{}", last.1, last.1[2],last.1[1],last.1[0]);
                }
            }
        }
    }
    println!("Should be: 296962999629");
}