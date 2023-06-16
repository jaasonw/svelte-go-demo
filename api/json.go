package handler

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

func Jsonparse(w http.ResponseWriter, r *http.Request) {
	var parsed map[string]interface{}
	res, err := http.Get("https://pokeapi.co/api/v2/pokemon?limit=100000&offset=0")
	if err != nil {
		log.Fatalln(err)
	}

	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		log.Fatalln(err)
	}
	jsonStr := string(body)

	start := time.Now()
	json.Unmarshal([]byte(jsonStr), &parsed)
	duration := time.Since(start)

	// fmt.Fprintf(w, "len=%d cap=%d %v\n", len(s), cap(s), s)
	fmt.Fprintf(w, "%f ms", float64(duration.Nanoseconds())/1000000)
}
