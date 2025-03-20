package cal

import "fmt"

func ExampleNaturalSequence() {
	n := 1
	for i := 0; i < 8; i++ {
		a := NaturalSequence(n)
		fmt.Printf("%.3f\t%.4f\t(n = %.e)\n", a, a, float64(n))
		n *= 10
	}

	// Output:
	// Output:
	// 2.000	2.0000	(n = 1e+00)
	// 2.594	2.5937	(n = 1e+01)
	// 2.705	2.7048	(n = 1e+02)
	// 2.717	2.7169	(n = 1e+03)
	// 2.718	2.7181	(n = 1e+04)
	// 2.718	2.7183	(n = 1e+05)
	// 2.718	2.7183	(n = 1e+06)
	// 2.718	2.7183	(n = 1e+07)
}
