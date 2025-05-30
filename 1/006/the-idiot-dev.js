let sumOf100 = 0
let sumOfSquare = 0
for(let i = 1; i <= 100; i++) {
    sumOf100 += i
    sumOfSquare += i ** 2
}

console.log('Result: ', sumOf100 ** 2 - sumOfSquare)
