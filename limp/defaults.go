package main

import "fmt"

func main() {
        	c := make(chan int, 3)
		c <- 1
        	c <- 2
	        c <- 3
	        close(c)
	        for i := 0; i < 10; i++ {
		                fmt.Printf("%d ", <-c)
	        }

		c = nil
		fmt.Printf("%d ", <-c)

}
