package main

import (
	"fmt"
	"math"
)

func main() {
	var sumSquare, squareSum float64 = 0, 0
	var i float64 = 1
	for i <= 100 {
		sumSquare += math.Pow(i, 2)
		squareSum += i
		i++
	}
	ans := math.Pow(squareSum, 2) - sumSquare
	fmt.Printf("%v", ans)
}