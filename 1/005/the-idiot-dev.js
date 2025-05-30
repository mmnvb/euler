let found = false
let n = 20
let nums = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
while(!found) {
    if(nums.every(num => n % num === 0)) found = true
    else n++
    
}

console.log(n)
