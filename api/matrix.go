package handler

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"

	"gonum.org/v1/gonum/mat"
)

func Test(w http.ResponseWriter, r *http.Request) {

	matrix_size := 300

	// Generate a 6×6 matrix of random values.
	data := make([]float64, matrix_size*matrix_size)
	for i := range data {
		data[i] = rand.NormFloat64()
	}
	a := mat.NewDense(matrix_size, matrix_size, data)

	data = make([]float64, matrix_size*matrix_size)
	for i := range data {
		data[i] = rand.NormFloat64()
	}
	b := mat.NewDense(matrix_size, matrix_size, data)

	c := mat.NewDense(matrix_size, matrix_size, nil)

	start := time.Now()
	c.Mul(a, b)
	duration := time.Since(start)

	// fmt.Fprintf(w, "len=%d cap=%d %v\n", len(s), cap(s), s)
	fmt.Fprintf(w, "%f ms", float64(duration.Nanoseconds())/1000000)
}
