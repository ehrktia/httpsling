package main

import (
	"errors"
	"fmt"
	"net"
	"net/http"
	"os"
)

func main() {
	addr := net.JoinHostPort("::", "8888")
	m := http.NewServeMux()
	var home http.HandlerFunc = func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("received home request")
		_, _ = w.Write([]byte("from home"))
	}
	m.Handle("/", home)
	h := http.Server{
		Addr:    addr,
		Handler: m,
	}
	errCh := make(chan error)
	fmt.Println("server running on port:8888")

	go func() {
		if err := h.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			errCh <- err
		}
	}()
	e := <-errCh
	fmt.Fprintf(os.Stderr, "error starting server in port 8888:%v\n", e)
}
