package handler

import (
	"fmt"
	"net/http"
	"time"
)

func Handler(w http.ResponseWriter, r *http.Request) {
	s := []string{}
	start := time.Now()
	for i := 0; i < 5000; i++ {
		s = append(s, "test")
	}
	duration := time.Since(start)
	// fmt.Fprintf(w, "len=%d cap=%d %v\n", len(s), cap(s), s)
	fmt.Fprintf(w, "%f ms", float64(duration.Nanoseconds())/1000000)
}
