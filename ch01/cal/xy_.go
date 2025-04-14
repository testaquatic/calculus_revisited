package cal

import (
	"image/png"
	logger "log"
	"os"
)

func (xy XY) Save() {
	fp, err := os.Create("image.png")
	if err != nil {
		logger.Fatal(err)
	}
	defer fp.Close()

	png.Encode(fp, xy.Image)
}
