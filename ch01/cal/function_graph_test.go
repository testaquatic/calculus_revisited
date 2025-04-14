package cal_test

import (
	"testing"

	"github.com/testaquatic/calculus_revisited/ch01/cal"
)

func TestFunction_Graph(t *testing.T) {
	f := cal.Function{
		Interval: [2]float64{-1, 1},
		Formula:  cal.Exp,
	}
	g := f.Graph()
	xy := cal.NewXY()
	xy.Xlim = [2]float64{-1.1, 1.1}
	xy.Ylim = [2]float64{0, 3}
	xy.Draw(g)
	xy.Save()
}
