package main

import "fmt"

func main() {
	for a := 1; a < 1000; a++ {
		for b := a + 1; b < 1000; b++ {
			c := 1000 - a - b
			if c > 0 && a * a + b * b == c * c {
				fmt.Printf("%v\n", a * b * c)
				return
			}
		}
	}
}