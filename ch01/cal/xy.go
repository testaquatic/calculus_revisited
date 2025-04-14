package cal

import "image"

type XY struct {
	// 이미지의 가로와 세로
	W, H int
	// x, y[min, max]
	Xlim, Ylim [2]float64
	// 이미지 포인터
	Image *image.Paletted
}
