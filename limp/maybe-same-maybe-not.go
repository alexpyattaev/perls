package main

import "fmt"

func main() {
	a := new(struct{})
	b := new(struct{})
	println(a == b)

	c := new(struct{})
	d := new(struct{})
	println(c == d)
	fmt.Println(c, d)
}
