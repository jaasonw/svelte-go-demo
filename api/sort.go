package handler

import (
	"fmt"
	"math/rand"
	"net/http"
	"sort"
	"time"
)

func Sort(w http.ResponseWriter, r *http.Request) {
	s := rand.Perm(5000000)
	start := time.Now()
	sort.Ints(s)
	duration := time.Since(start)
	// fmt.Fprintf(w, "len=%d cap=%d %v\n", len(s), cap(s), s)
	fmt.Fprintf(w, "%f ms", float64(duration.Nanoseconds())/1000000)
}
