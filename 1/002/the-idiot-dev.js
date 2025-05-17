let sum = 0
function* fib() {
    let curr = 1
    let next = 2
    while(true) {
        let temp = curr
        if(curr % 2 === 0) sum += curr
        yield curr
        curr = next
        next += temp
    }
}

const gen = fib()
while(true) {
    if(gen.next().value >= 4_000_000) break;
}

console.log('Result: ', sum)
