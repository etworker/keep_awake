package main

import (
	"bytes"
	"image"
	"image/color"
	"image/draw"
	"math"
)

func sunIcon(active bool) image.Image {
	const size = 32
	img := image.NewRGBA(image.Rect(0, 0, size, size))

	cx, cy := 15.5, 15.5

	var r, g, b uint8
	if active {
		r, g, b = 255, 183, 77
	} else {
		r, g, b = 158, 158, 158
	}

	rays := [][2]float64{
		{1.0, 0.0}, {0.707, 0.707}, {0.0, 1.0}, {-0.707, 0.707},
		{-1.0, 0.0}, {-0.707, -0.707}, {0.0, -1.0}, {0.707, -0.707},
	}

	for py := 0; py < size; py++ {
		for px := 0; px < size; px++ {
			dx := float64(px) - cx
			dy := float64(py) - cy
			dist := math.Sqrt(dx*dx + dy*dy)

			// Sun body: filled circle radius 8
			if dist <= 8.0 {
				alpha := uint8(255)
				if dist > 7.0 {
					alpha = uint8((8.0 - dist) * 255.0)
				}
				img.Set(px, py, color.RGBA{r, g, b, alpha})
				continue
			}

			// Rays: between radius 9 and 13
			if dist > 9.0 && dist <= 13.0 {
				for _, ray := range rays {
					rx, ry := ray[0], ray[1]
					perp := math.Abs(dx*ry - dy*rx)
					proj := dx*rx + dy*ry
					if perp <= 1.8 && proj >= 9.0 && proj <= 13.0 {
						alpha := uint8(255)
						if perp > 1.0 {
							alpha = uint8((1.8 - perp) * 255.0)
						}
						img.Set(px, py, color.RGBA{r, g, b, alpha})
						break
					}
				}
			}
		}
	}
	return img
}

func iconToRGBA(img image.Image) []byte {
	bounds := img.Bounds()
	rgba := image.NewRGBA(bounds)
	draw.Draw(rgba, bounds, img, bounds.Min, draw.Src)
	return rgba.Pix
}

func sunIconICO(active bool) []byte {
	img := sunIcon(active)
	bounds := img.Bounds()
	w, h := bounds.Dx(), bounds.Dy()

	// Convert RGBA to BGRA (ICO uses BGR order)
	pixels := iconToRGBA(img)
	bgra := make([]byte, len(pixels))
	for i := 0; i < len(pixels); i += 4 {
		bgra[i], bgra[i+1], bgra[i+2], bgra[i+3] = pixels[i+2], pixels[i+1], pixels[i], pixels[i+3]
	}

	// BMP info header (40 bytes)
	bmpHeader := make([]byte, 40)
	bmpHeader[0] = 40       // header size
	bmpHeader[4] = byte(w)  // width
	bmpHeader[5] = byte(w >> 8)
	bmpHeader[8] = byte(h * 2) // height (doubled for ICO)
	bmpHeader[9] = byte(h * 2 >> 8)
	bmpHeader[12] = 1       // planes
	bmpHeader[14] = 32      // bpp
	// No compression, no color table

	// AND mask (1bpp, scanlines padded to 4 bytes)
	andStride := ((w + 31) / 32) * 4
	andMask := make([]byte, andStride*h)

	// ICO directory entry (16 bytes)
	entry := make([]byte, 16)
	entry[0] = byte(w)      // width (0 means 256)
	entry[1] = byte(h)      // height
	entry[2] = 0            // colors
	entry[3] = 0            // reserved
	entry[4] = 1            // color planes
	entry[5] = 0
	entry[6] = 32           // bpp
	entry[7] = 0
	// Size
	totalSize := 40 + len(bgra) + len(andMask)
	entry[8] = byte(totalSize)
	entry[9] = byte(totalSize >> 8)
	entry[10] = byte(totalSize >> 16)
	entry[11] = byte(totalSize >> 24)
	// Offset (header 6 + entry 16 = 22)
	entry[12] = 22
	entry[13] = 0
	entry[14] = 0
	entry[15] = 0

	// ICO header (6 bytes): reserved=0, type=1, count=1
	header := []byte{0, 0, 1, 0, 1, 0}

	return bytes.Join([][]byte{header, entry, bmpHeader, bgra, andMask}, nil)
}

