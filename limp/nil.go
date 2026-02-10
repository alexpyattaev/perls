// go run nil.go
package main

type ICar interface {
	Start()
}

func getICar() ICar {
	return nil
}

func main() {
	c := getICar()
	c.Start() 
}

