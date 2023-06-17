package handler

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func Pi(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	const INTERVAL = 50000
	circlePoints := 0
	squarePoints := 0
	for i := 0; i < INTERVAL*INTERVAL; i++ {
		randX := rand.Float64() * INTERVAL
		randY := rand.Float64() * INTERVAL
		originDist := randX*randX + randY*randY

		if originDist <= 1 {
			circlePoints++
		}
		squarePoints++

		// pi := 4 * float64(circlePoints) / float64(squarePoints)
	}
	duration := time.Since(start)
	// fmt.Fprintf(w, "len=%d cap=%d %v\n", len(s), cap(s), s)
	fmt.Fprintf(w, "%f ms", float64(duration.Nanoseconds())/1000000)
}
