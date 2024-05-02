package main

import "fmt"

func main() {
	i := 1
	for {
		flag := true
		for j := 1; j <= 20; j++ {
			if i % j != 0 {
				flag = false
				break
			}
		}
		if flag {
			fmt.Printf("%v", i)
			break
		}
		i++
	}
}