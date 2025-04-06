package cal

import (
	"image/png"
	"os"
)

func (xy XY) Save() {
	fp, err := os.Create("image.png")
	if err != nil {
		panic(err)
	}
	defer fp.Close()

	png.Encode(fp, xy.Image)
}
