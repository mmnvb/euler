fn fac(n: u128, start:u128) -> u128{
    let mut result = 1;
    for i in start..=n{
        result *= i;
    }
    println!("{} при  {}", result, n);
    result
}

fn main(){
    // this is the size of the grid 
    let a = 20;
    let b = 20;

    let steps = a + b; // total number of step it takes to reach the right bottom in any way (since we have 21x21 points)

    // now we need to answer the question
    // in the case of 3x3 grid
    // we have 6 places and 3 ways of each direction. How many combinations are there considering that we don't want any duplications (order doesn't matters)
    // so we use basic combinatorics formula C(n, k)= n!/(n-k)! * k! , where n = total moves required, and k=limit for each direction

    // ultimate formula
    // let answer = fac(steps, 2)/fac(steps-a, 2)/fac(a, 2);

    // formula after algebraic stuff exactly for this case
    let answer = fac(steps, 21)/fac(a, 2);
    println!("Answer: {} steps", answer);
    println!("Should be: 137846528820");


    // since a and are equal we could remove var "b" , but for the other purposes with non squre grids that will fail
}