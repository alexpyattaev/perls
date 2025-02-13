package main

import (
        "fmt"
)

type Foo struct {}

func (f *Foo) bar() {
	fmt.Println("test")
}

func main() {
	foo := (*Foo)(nil) // nil pointer to Foo
	foo.bar()
}

