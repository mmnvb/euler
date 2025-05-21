function pythagoreanTriplet(n) {
    for(let i = 1; i < n; i++) {
        for(let j = 1; j < n; j++) {
            for(let k = 1; k < n; k++) {
                if(i + j + k == n && i < j && j < k && ((i ** 2) + (j ** 2) == (k ** 2))) {
                    return [i, j, k]
                }
            }
        }
    }
    return 'failed!!!'
}

console.log(pythagoreanTriplet(1000))
