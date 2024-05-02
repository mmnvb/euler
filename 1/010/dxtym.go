package main

import "fmt"

func sieve(lim int) []bool {
	sieve := make([]bool, lim)

	for i := 2; i < lim; i++ {
		sieve[i] = true
	}

	for i := 2; i * i < lim; i++ {
		if sieve[i] {
			for j := i * i; j < lim; j += i {
				sieve[j] = false
			}
		}
	}

	return sieve
}

func main() {
	sum := 0
	sieve := sieve(2000000)
	for i, j := range sieve {
		if j {
			sum += i
		}
	}
	fmt.Println(sum)
}

// Source: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes