package main

import "fmt"

func main() {
	var n int = 1000
	var f, y float64 = 1 + 1/float64(n), 1
	for i := 0; i < n; i++ {
		y = y * f
	}

	fmt.Println(y)
}
