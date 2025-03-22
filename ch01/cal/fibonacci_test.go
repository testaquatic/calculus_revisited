package cal

import "testing"

func TestFibonacci(t *testing.T) {
	if Fibonacci(10) != 89 {
		t.Error("Fibonacci(10) != 89")
	}
}
