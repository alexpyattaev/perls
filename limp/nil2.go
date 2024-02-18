// go run nil2.go

package main


import (
        "fmt"
)


type ICar interface {
	Start()
}

func getICar() ICar {
	return getCar()
}

type Car struct { // no need to explicitly say 'implements ICar'
	sound string
}

func (c *Car) Start() { // because Car has a Start() function it implements ICar
	fmt.Println(c.sound)
}

func getCar() *Car {
	return nil
}

func main() {
	c := getICar()
	if c == nil {
		fmt.Println("exiting")
		return
	}
	c.Start() 
}

