package cal

func ExpSequence(x float64, n int) float64 {
	if n < 0 {
		return 0
	}

	d := BaseN(n, 2)
	e, f := 1.0, 1.0+x/float64(n)
	for i, k := range d {
		if k == 1 {
			e *= InterativeSquare(f, i)
		}
	}

	return e
}

func Exp(x float64) float64 {
	f, p := 1.0, 1.0
	for i := 0; !IsZero(p); i++ {
		p *= x / float64(i+1)
		f += p
	}

	return f
}
