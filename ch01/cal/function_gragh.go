package cal

const NSUB = 100

func (fn Function) Graph() Graph {
	a, b := fn.Interval[0], fn.Interval[1]
	xarr := Linspace(a, b, NSUB)
	var graph Graph
	for i := 0; i < NSUB; i++ {
		x0, x1 := xarr[i], xarr[i+1]
		y0, y1 := fn.Formula(x0), fn.Formula(x1)
		graph.Lines = append(graph.Lines, Line{Point{x0, y0}, Point{x1, y1}})
	}

	return graph
}


