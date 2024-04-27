package main

import "fmt"

func isPrime(x int) bool {
	y := x
	for y > 0 {
		if x % y == 0 && (y != x && y != 1) {
			return false
		} 
		y--
	}
	return true
}

func main() {
	i := 1
	j := 2
	for i != 10001 {
		if isPrime(j) {
			i++
		}
		j++
	}
	fmt.Println(j)
}