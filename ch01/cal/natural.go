package cal

func InterativeSquare(a float64, n int) float64 {
	x := a
	for i := 0; i < n; i++ {
		x = x * x
	}

	return x
}

func NaturalSequence(n int) float64 {
	e, f := 1.0, 1+1/float64(n)
	d := BaseN(n, 2)
	for i, k := range d {
		if k == 1 {
			e *= InterativeSquare(f, i)
		}
	}

	return e
}
