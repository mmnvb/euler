function isPrime(n) {
    if(n <= 1) return false
    if(n == 2) return true
    if(n % 2 == 0) return false
    for(let i = 3; i <= Math.sqrt(n); i+=2) {
        if(n % i == 0) {
            return false
        }
    }
    return true
}
let primeNum
let primeNumCounter = 0
let num = 1
while(true) {
    if(isPrime(num)) {
        primeNumCounter++
        primeNum = num
        if(primeNumCounter === 10_001) break;
    }
    num++
}

console.log(primeNum)
