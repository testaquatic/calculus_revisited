package cal

import "image"

type XY struct {
	W, H       int
	Xlim, Ylim [2]float64
	Image      *image.Paletted
}



